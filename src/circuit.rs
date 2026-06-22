use ndarray::{Array2, ArrayD};
use num_complex::Complex64;

use crate::constants::{gate_cnot, gate_cz, gate_h, gate_s, gate_t, gate_x, gate_y, gate_z};
use crate::engine::phase::{apply_diffusion_in_place, apply_phase_on_basis_match};
use crate::ops::{apply_controlled_single_qubit_gate_inplace, apply_k_qubit_gate_inplace};
use crate::utils::q0_n;

enum Operation {
    Gate {
        gate: Array2<Complex64>,
        targets: Vec<usize>,
    },
    ControlledGate {
        gate: Array2<Complex64>,
        controls: Vec<usize>,
        target: usize,
    },
    PhaseOnBasisMatch {
        mask: usize,
        pattern: usize,
        phase: Complex64,
    },
    Diffusion,
}

pub struct Circuit {
    n_qubits: usize,
    operations: Vec<Operation>,
}

impl Circuit {
    pub fn new(n_qubits: usize) -> Self {
        assert!(n_qubits > 0, "Circuit must contain at least one qubit");
        Self {
            n_qubits,
            operations: Vec::new(),
        }
    }

    /// Adds a gate operation to the circuit without executing it immediately.
    ///
    /// The gate is stored together with its target qubits and will be applied later
    /// when `run()` is called. The gate matrix must have the shape `2^k × 2^k`,
    /// where `k = targets.len()`.
    pub fn add_gate(&mut self, gate: Array2<Complex64>, targets: &[usize]) -> &mut Self {
        assert!(!targets.is_empty(), "Gate must target at least one qubit");
        for &target in targets {
            assert!(
                target < self.n_qubits,
                "Target qubit {} is out of range for {}-qubit circuit",
                target,
                self.n_qubits
            );
        }

        for i in 0..targets.len() {
            for j in (i + 1)..targets.len() {
                assert_ne!(
                    targets[i], targets[j],
                    "Duplicate target qubit {}",
                    targets[i]
                );
            }
        }

        let dim = 1 << targets.len();
        assert_eq!(
            gate.shape(),
            &[dim, dim],
            "Gate shape must be {}x{} for {} target qubits",
            dim,
            dim,
            targets.len()
        );

        self.operations.push(Operation::Gate {
            gate,
            targets: targets.to_vec(),
        });

        self
    }

    fn add_controlled_gate(
        &mut self,
        gate: Array2<Complex64>,
        controls: &[usize],
        target: usize,
    ) -> &mut Self {
        self.operations.push(Operation::ControlledGate {
            gate,
            controls: controls.to_vec(),
            target,
        });

        self
    }

    pub fn h(&mut self, target: usize) -> &mut Self {
        self.add_gate(gate_h(), &[target])
    }

    pub fn x(&mut self, target: usize) -> &mut Self {
        self.add_gate(gate_x(), &[target])
    }

    pub fn y(&mut self, target: usize) -> &mut Self {
        self.add_gate(gate_y(), &[target])
    }

    pub fn z(&mut self, target: usize) -> &mut Self {
        self.add_gate(gate_z(), &[target])
    }

    pub fn s(&mut self, target: usize) -> &mut Self {
        self.add_gate(gate_s(), &[target])
    }

    pub fn t(&mut self, target: usize) -> &mut Self {
        self.add_gate(gate_t(), &[target])
    }

    pub fn h_all(&mut self) -> &mut Self {
        for qubit in 0..self.n_qubits {
            self.h(qubit);
        }

        self
    }

    pub fn cnot(&mut self, control: usize, target: usize) -> &mut Self {
        self.add_gate(gate_cnot(), &[control, target])
    }

    pub fn cz(&mut self, control: usize, target: usize) -> &mut Self {
        self.add_gate(gate_cz(), &[control, target])
    }

    pub fn mcx(&mut self, controls: &[usize], target: usize) -> &mut Self {
        self.add_controlled_gate(gate_x(), controls, target)
    }

    pub fn mcz(&mut self, controls: &[usize], target: usize) -> &mut Self {
        self.add_controlled_gate(gate_z(), controls, target)
    }

    /// Marks a basis state by flipping the sign of its amplitude.
    /// This is a direct state-vector phase marking operation:
    /// the amplitude changes sign, while its measurement probability is unchanged.
    pub fn phase_oracle(&mut self, target_index: usize) -> &mut Self {
        assert!(
            target_index < (1usize << self.n_qubits),
            "Target index {} is out of range for {} qubits",
            target_index,
            self.n_qubits
        );

        let full_mask = (1usize << self.n_qubits) - 1;

        self.operations.push(Operation::PhaseOnBasisMatch {
            mask: full_mask,
            pattern: target_index,
            phase: Complex64::new(-1.0, 0.0),
        });

        self
    }

    /// Reflects all amplitudes around their mean.
    /// This is the diffusion step used in Grover search and amplitude amplification:
    /// it turns phase marking into increased measurement probability.
    pub fn diffusion(&mut self) -> &mut Self {
        self.operations.push(Operation::Diffusion);
        self
    }

    /// Executes all scheduled gate operations on the initial `|0...0>` state.
    ///
    /// Operations are applied sequentially in the order they were added to the
    /// circuit. The execution uses the generic in-place k-qubit gate engine.
    pub fn run(&self) -> ArrayD<Complex64> {
        let mut state = q0_n(self.n_qubits);

        for operation in &self.operations {
            match operation {
                Operation::Gate { gate, targets } => {
                    apply_k_qubit_gate_inplace(&mut state, gate, targets);
                }
                Operation::ControlledGate {
                    gate,
                    controls,
                    target,
                } => {
                    apply_controlled_single_qubit_gate_inplace(&mut state, gate, controls, *target);
                }
                Operation::PhaseOnBasisMatch {
                    mask,
                    pattern,
                    phase,
                } => {
                    apply_phase_on_basis_match(&mut state, self.n_qubits, *mask, *pattern, *phase);
                }
                Operation::Diffusion => {
                    apply_diffusion_in_place(&mut state);
                }
            }
        }

        state
    }
}

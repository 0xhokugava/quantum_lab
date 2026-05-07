use ndarray::{Array2, ArrayD};
use num_complex::Complex64;

use crate::constants::{gate_cnot, gate_h, gate_s, gate_t, gate_x, gate_y, gate_z};
use crate::ops::apply_k_qubit_gate_inplace;
use crate::utils::q0_n;

struct Operation {
    gate: Array2<Complex64>,
    targets: Vec<usize>,
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

        self.operations.push(Operation {
            gate,
            targets: targets.to_vec(),
        });

        self
    }

    pub fn h(&mut self, target: usize) -> &mut Self {
        self.add_gate(gate_h(), &[target])
    }

    pub fn cnot(&mut self, target: usize, control: usize) -> &mut Self {
        self.add_gate(gate_cnot(), &[target, control])
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

    pub fn run(&self) -> ArrayD<Complex64> {
        let mut state = q0_n(self.n_qubits);

        for operation in &self.operations {
            apply_k_qubit_gate_inplace(&mut state, &operation.gate, &operation.targets);
        }

        state
    }
}

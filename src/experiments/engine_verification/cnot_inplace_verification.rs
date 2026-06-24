use crate::engine::constants::{gate_cnot, gate_h, q0, q1};
use crate::engine::ops::{apply_cnot_inplace, tensor_product};
use crate::engine::utils::{assert_states_close, to_dirac};

/// Runs a verification experiment for the in-place CNOT implementation.
///
/// This experiment compares the matrix-based CNOT application with the
/// optimized in-place version across a set of representative input states:
/// - Computational basis states (|00⟩, |10⟩)
/// - A superposition state (H|0⟩ ⊗ |0⟩)
///
/// For each input:
/// - The expected result is computed using standard matrix multiplication
/// - The in-place result is computed using bitmask-based logic
/// - Both results are compared for numerical equivalence
///
/// The superposition case demonstrates the creation of entanglement,
/// producing the Bell state |Φ⁺⟩ = (|00⟩ + |11⟩)/√2.
///
/// This serves as both a correctness check and a physical interpretation
/// of the CNOT operation within the simulator.
pub fn run() {
    println!("\nCNOT In-place Verification (with Superposition):");

    let control = 1;
    let target = 0;
    let hq0 = gate_h().dot(&q0());

    let test_states = vec![
        ("|00>", tensor_product(&q0(), &q0())),
        ("|10>", tensor_product(&q1(), &q0())),
        ("H|0> ⊗ |0>", tensor_product(&hq0, &q0())),
    ];

    let matrix = gate_cnot();

    for (label, state_initial) in test_states {
        println!("\n   Input state {}:", label);
        println!("   Before : {}", to_dirac(&state_initial));

        let state_vec = state_initial
            .clone()
            .into_dimensionality::<ndarray::Ix1>()
            .unwrap();

        let expected_state = matrix.dot(&state_vec).into_dyn();
        let mut state_inplace = state_initial.clone();

        apply_cnot_inplace(&mut state_inplace, control, target);
        assert_states_close(&state_inplace, &expected_state);

        println!("   After  : {}", to_dirac(&state_inplace));
        if label == "H|0> ⊗ |0" {
            println!("   → Bell State |Φ⁺> = (|00> + |11>)/√2");
        }
    }
    println!("\n   [SUCCESS] CNOT verified on basis and superposition.\n");
}

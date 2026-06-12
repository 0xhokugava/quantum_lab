use crate::constants::{gate_h, identity};
use crate::ops::{apply_gate_inplace, tensor_product};
use crate::utils::{assert_states_close, q0_n, to_dirac};

pub fn run() {
    println!("\nIn-place Gate Application Verification:\n");
    let n_qubits = 3;
    let state_initial = q0_n(n_qubits);
    let target_qubit = 1;
    let gate = gate_h();

    println!(
        "   System size: {} qubits ({} amplitudes)",
        n_qubits,
        state_initial.len()
    );
    println!("   Target qubit: {}", target_qubit);

    // Construct the global operator (I ⊗ H ⊗ I) manually to serve as the ground truth
    // Printed basis labels use standard binary order: |q2 q1 q0>.
    // Qubit 0 is still the least significant bit internally.
    let mut full_matrix = identity().into_dyn();
    full_matrix = tensor_product(&gate, &full_matrix);
    full_matrix = tensor_product(&identity(), &full_matrix);

    let matrix_2d = full_matrix.into_dimensionality::<ndarray::Ix2>().unwrap();
    let state_vec = state_initial
        .clone()
        .into_dimensionality::<ndarray::Ix1>()
        .unwrap();
    let expected_state = matrix_2d.dot(&state_vec).into_dyn();

    // Apply the gate directly to the state vector using bit-masking logic.
    // This avoids large matrix allocations and redundant zero-multiplications.
    let mut state_inplace = state_initial.clone();
    apply_gate_inplace(&mut state_inplace, &gate, target_qubit);

    // Compare the optimized result with the matrix multiplication baseline.
    // Use a high-precision delta (1e-15) to ensure numerical consistency.

    assert_states_close(&state_inplace, &expected_state);

    println!("   [SUCCESS] In-place result matches matrix multiplication perfectly.");
    println!("   Resulting state: {}", to_dirac(&state_inplace));
}

use crate::constants::{gate_h, identity};
use crate::ops::{apply_gate_inplace, tensor_product};
use crate::utils::{q0_n, to_dirac};

pub fn run() {
    println!("\n5. In-place Gate Application Verification");
    // Initialize a 3-qubit system in the |000> state
    let n_qubits = 3;
    let state_initial = q0_n(n_qubits);
    // Middle qubit
    let target_qubit = 1;
    let gate = gate_h();

    println!(
        "   System size: {} qubits ({} amplitudes)",
        n_qubits,
        state_initial.len()
    );
    println!("   Target qubit: {}", target_qubit);

    // Construct the global operator (I ⊗ H ⊗ I) manually to serve as the ground truth
    // Order: Leftmost (q2) -> Target (q1) -> Rightmost (q0) for Little-endian.
    let mut full_matrix = identity().into_dyn();
    full_matrix = tensor_product(&gate, &full_matrix);
    full_matrix = tensor_product(&identity(), &full_matrix);

    // Convert to 2D matrix and 1D vector for standard multiplication
    let matrix_2d = full_matrix.into_dimensionality::<ndarray::Ix2>().unwrap();
    let state_vec = state_initial
        .clone()
        .into_dimensionality::<ndarray::Ix1>()
        .unwrap();
    let expected_state = matrix_2d.dot(&state_vec);

    // Apply the gate directly to the state vector using bit-masking logic.
    // This avoids large matrix allocations and redundant zero-multiplications.
    let mut state_inplace = state_initial.clone();
    apply_gate_inplace(&mut state_inplace, &gate, target_qubit);

    // Compare the optimized result with the matrix multiplication baseline.
    // Use a high precision delta (1e-15) to ensure numerical consistency.
    let mut is_correct = true;
    for (i, (a, b)) in state_inplace.iter().zip(expected_state.iter()).enumerate() {
        if (a - b).norm() > 1e-15 {
            println!(
                "   [ERROR] Mismatch at index {}: expected {}, got {}",
                i, b, a
            );
            is_correct = false;
            break;
        }
    }

    if is_correct {
        println!("   [SUCCESS] In-place result matches matrix multiplication perfectly.");
        println!("   Resulting state: {}", to_dirac(&state_inplace));
    }
}

use ndarray::{Array1, array};
use quantum_lab::formatting::to_dirac;

#[test]
fn test_to_dirac_formatting() {
    // 1 Qubit: length 2, binary width should be 1 (|0>)
    let q1 = array![1.0, 0.0];
    assert_eq!(to_dirac(&q1), "+1.000|0>");

    // 2 Qubits: length 4, binary width should be 2 (|00>)
    let q2 = array![0.0, 0.0, 1.0, 0.0]; // state |10>
    assert_eq!(to_dirac(&q2), "+1.000|10>");

    // 3 Qubits: length 8, binary width should be 3 (|000>)
    let mut q3 = Array1::zeros(8);
    q3[7] = 1.0; // state |111>
    assert_eq!(to_dirac(&q3), "+1.000|111>");
}

#[test]
fn test_to_dirac_superposition() {
    // Test filtering and formatting for multiple states (H|0>)
    let h_state = array![
        std::f64::consts::FRAC_1_SQRT_2,
        std::f64::consts::FRAC_1_SQRT_2
    ];
    let result = to_dirac(&h_state);
    // Should show both states with their amplitudes
    assert!(result.contains("|0>"));
    assert!(result.contains("|1>"));
}

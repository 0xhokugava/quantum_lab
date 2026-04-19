use ndarray::array;
use num_complex::Complex64;
use quantum_lab::constants::{gate_cnot, gate_h, gate_x, gate_z, identity, q0, q1};

fn to_c64(re: f64) -> Complex64 {
    Complex64::new(re, 0.0)
}

#[test]
fn test_gate_x_logic() {
    // X|0> = |1>
    let result = gate_x().dot(&q0());
    assert_eq!(result, &q1());
}

#[test]
fn test_gate_h_reversibility() {
    // H(H|0>) = |0>
    let once = gate_h().dot(&q0());
    let twice = gate_h().dot(&once);

    // Use tolerance check for float precision
    for (a, b) in twice.iter().zip(q0().iter()) {
        assert!((a - b).norm() < 1e-10);
    }
}

#[test]
fn test_gate_h_is_unitary() {
    // Norm of H|0> must be 1.0
    let result = gate_h().dot(&q0());
    let norm: f64 = result.mapv(|x| x.norm_sqr()).sum();
    assert!((norm - 1.0).abs() < 1e-10);
}

#[test]
fn test_cnot_logic() {
    let input = array![to_c64(0.0), to_c64(0.0), to_c64(1.0), to_c64(0.0),];

    let output = gate_cnot().dot(&input);

    let expected = array![to_c64(0.0), to_c64(0.0), to_c64(0.0), to_c64(1.0),];

    assert_eq!(output, expected);

    let input_01 = array![to_c64(0.0), to_c64(1.0), to_c64(0.0), to_c64(0.0),];

    let output_01 = gate_cnot().dot(&input_01);
    assert_eq!(output_01, input_01);
}

#[test]
fn test_identity_logic() {
    assert_eq!(identity().dot(&q0()), q0());
}

#[test]
fn test_gate_z_logic() {
    let input = array![to_c64(0.0), to_c64(1.0)];
    let output = gate_z().dot(&input);
    let expected = array![to_c64(0.0), to_c64(-1.0)];
    for (a, b) in output.iter().zip(expected.iter()) {
        assert!((*a - *b).norm() < 1e-10);
    }
}

#[test]
fn test_gate_z_preserves_probabilities() {
    let state = gate_h().dot(&q0());
    let state_z = gate_z().dot(&state);

    let probs_before: Vec<f64> = state.iter().map(|x| x.norm_sqr()).collect();
    let probs_after: Vec<f64> = state_z.iter().map(|x| x.norm_sqr()).collect();

    for (a, b) in probs_before.iter().zip(probs_after.iter()) {
        assert!((a - b).abs() < 1e-10);
    }
}

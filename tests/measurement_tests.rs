use ndarray::array;
use quantum_lab::constants::{q0, q1};
use quantum_lab::measurement::{measure, test_measure};
use std::f64::consts::FRAC_1_SQRT_2;

#[test]
fn test_measure_deterministic() {
    // |0> must always collapse to 0
    for _ in 0..1000 {
        assert_eq!(measure(&q0()), 0);
    }

    // |1> must always collapse to 1
    for _ in 0..1000 {
        assert_eq!(measure(&q1()), 1);
    }
}

#[test]
fn test_measure_statistics() {
    let h_state = array![FRAC_1_SQRT_2, FRAC_1_SQRT_2];
    let shots = 1000;
    let (p1, p0) = test_measure(&h_state, shots);

    // For Hadamard expectations roughly 50/50
    // Check if it's within ±10% to avoid rare random fails
    assert!(p1 > 40.0 && p1 < 60.0);
    assert!(p0 > 40.0 && p0 < 60.0);

    // Sum must be exactly 100% or very close
    assert!((p1 + p0 - 100.0).abs() < 1e-9);
}

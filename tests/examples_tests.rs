use num_complex::Complex64;
use quantum_lab::circuit::catalog::{BellState, bell};

fn assert_amplitude(actual: Complex64, expected: Complex64) {
    const EPSILON: f64 = 1e-10;

    assert!(
        (actual - expected).norm() < EPSILON,
        "expected {expected}, got {actual}"
    );
}
#[test]
fn builds_phi_plus() {
    let circuit = bell(BellState::PhiPlus);
    let state = circuit.run();

    assert_amplitude(
        state[0],
        Complex64::new(std::f64::consts::FRAC_1_SQRT_2, 0.0),
    );
    assert_amplitude(state[1], Complex64::new(0.0, 0.0));
    assert_amplitude(state[2], Complex64::new(0.0, 0.0));
    assert_amplitude(
        state[3],
        Complex64::new(std::f64::consts::FRAC_1_SQRT_2, 0.0),
    );
}

#[test]
fn builds_phi_minus() {
    let circuit = bell(BellState::PhiMinus);
    let state = circuit.run();

    assert_amplitude(
        state[0],
        Complex64::new(std::f64::consts::FRAC_1_SQRT_2, 0.0),
    );
    assert_amplitude(state[1], Complex64::new(0.0, 0.0));
    assert_amplitude(state[2], Complex64::new(0.0, 0.0));
    assert_amplitude(
        state[3],
        -Complex64::new(std::f64::consts::FRAC_1_SQRT_2, 0.0),
    );
}

#[test]
fn builds_psi_plus() {
    let circuit = bell(BellState::PsiPlus);
    let state = circuit.run();

    assert_amplitude(state[0], Complex64::new(0.0, 0.0));
    assert_amplitude(
        state[1],
        Complex64::new(std::f64::consts::FRAC_1_SQRT_2, 0.0),
    );
    assert_amplitude(
        state[2],
        Complex64::new(std::f64::consts::FRAC_1_SQRT_2, 0.0),
    );
    assert_amplitude(state[3], Complex64::new(0.0, 0.0));
}

#[test]
fn builds_psi_minus() {
    let circuit = bell(BellState::PsiMinus);
    let state = circuit.run();

    assert_amplitude(state[0], Complex64::new(0.0, 0.0));
    assert_amplitude(
        state[1],
        -Complex64::new(std::f64::consts::FRAC_1_SQRT_2, 0.0),
    );
    assert_amplitude(
        state[2],
        Complex64::new(std::f64::consts::FRAC_1_SQRT_2, 0.0),
    );
    assert_amplitude(state[3], Complex64::new(0.0, 0.0));
}

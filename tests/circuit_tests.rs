use ndarray::ArrayD;
use num_complex::Complex64;
use quantum_lab::circuit::Circuit;
use quantum_lab::constants::{q0, q1};
use quantum_lab::utils::assert_states_close;

#[test]
fn test_circuit_bell_state() {
    let mut circuit = Circuit::new(2);

    circuit.h(0);
    circuit.cnot(0, 1);

    let result = circuit.run();

    let inv_sqrt2 = 1.0 / 2.0_f64.sqrt();

    let expected = ArrayD::from_shape_vec(
        ndarray::IxDyn(&[4]),
        vec![
            Complex64::new(inv_sqrt2, 0.0), // |00>
            Complex64::new(0.0, 0.0),       // |01>
            Complex64::new(0.0, 0.0),       // |10>
            Complex64::new(inv_sqrt2, 0.0), // |11>
        ],
    )
    .unwrap();

    assert_states_close(&result, &expected);
}
#[test]
fn test_circuit_applies_operations_in_order() {
    let mut circuit = Circuit::new(1);
    let result = circuit.h(0).h(0).run();
    let expected = q0().into_dyn();
    assert_states_close(&result, &expected);
}
#[test]
fn test_circuit_x_gate() {
    let mut circuit = Circuit::new(1);
    let result = circuit.x(0).run();
    let expected = q1().into_dyn();
    assert_states_close(&result, &expected);
}
#[test]
fn test_circuit_chained_calls() {
    let mut circuit = Circuit::new(1);
    let result = circuit.x(0).x(0).run();
    let expected = q0().into_dyn();
    assert_states_close(&result, &expected);
}
#[test]
fn test_cz_basis_state() {
    let mut circuit = Circuit::new(2);
    circuit.x(0).x(1).cz(0, 1).run();
}
#[test]
fn test_cz_superposition() {
    let mut circuit = Circuit::new(2);
    circuit.h(0).h(1).cz(0, 1).run();
}

#[test]
fn mcx_applies_when_controls_are_one() {
    let mut circuit = Circuit::new(3);
    circuit.x(0).x(1).mcx(&[0, 1], 2);
    let state = circuit.run();
    assert_eq!(state[7], Complex64::new(1.0, 0.0));
}

#[test]
fn mcz_applies_phase_when_controls_are_one() {
    let mut circuit = Circuit::new(3);
    circuit.x(0).x(1).x(2).mcz(&[0, 1], 2);
    let state = circuit.run();
    assert_eq!(state[7], Complex64::new(-1.0, 0.0));
}

use ndarray::ArrayD;
use num_complex::Complex64;
use quantum_lab::circuit::Circuit;
use quantum_lab::constants::{q0, q1};
use quantum_lab::utils::assert_states_close;

#[test]
fn test_circuit_bell_state() {
    let mut circuit = Circuit::new(2);

    circuit.h(1);
    circuit.cnot(1, 0);

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

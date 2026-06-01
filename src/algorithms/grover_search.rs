use crate::circuit::Circuit;
use crate::utils::to_dirac;
use ndarray::{Array1, ArrayD};
use num_complex::Complex64;

#[derive(Debug)]
pub struct GroverResult {
    pub final_state: ArrayD<Complex64>,
    pub target_probability: f64,
    pub target_index: usize,
}

fn phase_oracle(state: &mut ArrayD<Complex64>, target_index: usize) {
    assert!(target_index < state.len());
    state[target_index] *= Complex64::new(-1.0, 0.0);
}

fn diffusion(state: &ArrayD<Complex64>) -> ArrayD<Complex64> {
    let ampl_sum: Complex64 = state.iter().sum();
    let mean = ampl_sum / Complex64::new(state.len() as f64, 0.0);

    state
        .iter()
        .map(|ampl| (2.0 * mean) - *ampl)
        .collect::<Array1<Complex64>>()
        .into_dyn()
}

pub fn run_grover(num_qubits: usize, target_index: usize) -> GroverResult {
    assert_eq!(num_qubits, 2);
    assert!(target_index < (1usize << num_qubits));

    let mut circuit = Circuit::new(num_qubits);
    circuit.h(0);
    circuit.h(1);
    let mut state = circuit.run();
    phase_oracle(&mut state, target_index);
    let new_state = diffusion(&state);

    let probability = new_state[target_index].norm_sqr();
    assert!((probability - 1.0).abs() < 1e-9);

    GroverResult {
        final_state: new_state,
        target_probability: probability,
        target_index,
    }
}

pub fn run_grover_2_qubit_demo() {
    let grover = run_grover(2, 2);

    println!("9. Grover search algorithm\n");

    println!("Final state: {}", to_dirac(&grover.final_state));
    println!("Target index: {}", grover.target_index);
    println!("Target probability: {:.6}", grover.target_probability);
}

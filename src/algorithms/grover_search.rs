use crate::circuit::Circuit;
use crate::utils::to_dirac;
use ndarray::{Array1, ArrayD};
use num_complex::Complex64;
use std::f64::consts::PI;

#[derive(Debug)]
pub struct GroverResult {
    pub final_state: ArrayD<Complex64>,
    pub target_probability: f64,
    pub target_index: usize,
    pub grover_steps: usize,
}

fn recommended_grover_steps(num_qubits: usize) -> usize {
    let search_space_size = 2.0_f64.powi(num_qubits as i32);
    ((PI / 4.0) * search_space_size.sqrt()).floor() as usize
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
    assert!(num_qubits > 0);
    assert!(target_index < (1usize << num_qubits));

    let mut circuit = Circuit::new(num_qubits);
    for qubit in 0..num_qubits {
        circuit.h(qubit);
    }
    let mut state = circuit.run();
    let grover_steps = recommended_grover_steps(num_qubits);

    for _ in 0..grover_steps {
        phase_oracle(&mut state, target_index);
        state = diffusion(&state);
    }

    let final_state = state;
    let target_probability = final_state[target_index].norm_sqr();

    GroverResult {
        final_state,
        target_probability,
        target_index,
        grover_steps,
    }
}

pub fn run_grover_demo() {
    let grover = run_grover(3, 5);

    println!("9. Grover search algorithm\n");

    println!("   Final state: {}", to_dirac(&grover.final_state));
    println!("   Target index: {}", grover.target_index);
    println!("   Target probability: {:.6}", grover.target_probability);
    println!("   Grover steps: {}", grover.grover_steps);
}

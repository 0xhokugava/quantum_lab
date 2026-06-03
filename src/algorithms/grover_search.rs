use crate::circuit::Circuit;
use crate::engine::phase::apply_phase_oracle_in_place;
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
        apply_phase_oracle_in_place(&mut state, num_qubits, target_index);
        state = diffusion(&state);
    }

    let target_probability = state[target_index].norm_sqr();

    GroverResult {
        final_state: state,
        target_probability,
        target_index,
        grover_steps,
    }
}

fn format_basis(index: usize, num_qubits: usize) -> String {
    format!("|{:0width$b}>", index, width = num_qubits)
}

pub fn run_grover_demo() {
    let num_qubits = 3;
    let target_index = 5;
    let grover = run_grover(num_qubits, target_index);

    println!("9. Grover search algorithm\n");

    println!("   Final state: {}", to_dirac(&grover.final_state));
    println!(
        "   Target state: {} index={}",
        format_basis(grover.target_index, num_qubits),
        grover.target_index
    );
    println!("   Target probability: {:.6}", grover.target_probability);
    println!("   Grover steps: {}", grover.grover_steps);
}

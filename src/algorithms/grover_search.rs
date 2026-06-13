use crate::circuit::Circuit;
use ndarray::ArrayD;
use num_complex::Complex64;
use std::f64::consts::PI;

// Grover search algorithm demo.
//
// This module implements Grover search for a single marked basis state.
// The circuit prepares a uniform superposition, applies a phase oracle for
// the target state, and then applies diffusion for the recommended number
// of Grover steps.
//
// The final result reports:
// - final state vector
// - marked target index
// - target measurement probability
// - number of Grover steps used

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

pub fn run_grover(num_qubits: usize, target_index: usize) -> GroverResult {
    assert!(num_qubits > 0);
    assert!(target_index < (1usize << num_qubits));

    let grover_steps = recommended_grover_steps(num_qubits);

    let mut circuit = Circuit::new(num_qubits);
    circuit.h_all();

    for _ in 0..grover_steps {
        circuit.phase_oracle(target_index);
        circuit.diffusion();
    }

    let final_state = circuit.run();
    let target_probability = final_state[target_index].norm_sqr();

    GroverResult {
        final_state,
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

    println!("\nGrover search algorithm:\n");

    // println!("   Final state: {}", to_dirac(&grover.final_state));
    println!(
        "   Target state: {} index={}",
        format_basis(grover.target_index, num_qubits),
        grover.target_index
    );
    println!("   Target probability: {:.6}", grover.target_probability);
    println!("   Grover steps: {}", grover.grover_steps);
}

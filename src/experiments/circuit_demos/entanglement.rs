use crate::circuit::catalog;
use crate::circuit::catalog::BellState;
use crate::engine::measurement::test_measure;
use crate::engine::utils::{decode_measurement, to_dirac};

pub fn run() {
    println!("\nQuantum Entanglement (The Bell State):\n");

    let n_qubits = 2;
    let shots = 100_000;

    let circuit = catalog::bell(BellState::PhiPlus);

    let bell_state = circuit
        .run()
        .into_dimensionality::<ndarray::Ix1>()
        .expect("Circuit output must be a 1D state vector");

    println!("   Bell State: {}", to_dirac(&bell_state));

    let stats = test_measure(&bell_state, shots);

    let mut sorted_indices: Vec<_> = stats.keys().copied().collect();
    sorted_indices.sort_unstable();

    for index in sorted_indices {
        let percentage = stats.get(&index).unwrap_or(&0.0);
        let bit_string = decode_measurement(index, n_qubits);
        println!("    |{}>: {:.2}%", bit_string, percentage);
    }

    println!("\n   Analysis:");
    if stats.len() == 2 && stats.contains_key(&0) && stats.contains_key(&(bell_state.len() - 1)) {
        println!("   Perfect correlation detected: states |00> and |11> share the probability.");
        println!("   This confirms the expected measurement correlation of the Bell state.");
    } else if stats.len() > 2 {
        println!("   State is in a multi-state superposition.");
    }
}

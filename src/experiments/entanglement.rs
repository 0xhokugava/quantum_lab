use crate::constants::{gate_cnot, gate_h, identity, q0};
use crate::formatting::{decode_measurement, to_dirac};
use crate::measurement::test_measure;
use crate::ops::tensor_product;

pub fn entanglement() {
    println!("\n4. Quantum Entanglement (The Bell State):");

    let bell_init = tensor_product(&q0(), &q0())
        .into_dimensionality::<ndarray::Ix1>()
        .expect("Initialization failed");

    let h_2q = tensor_product(&gate_h(), &identity())
        .into_dimensionality::<ndarray::Ix2>()
        .expect("H-gate matrix creation failed");

    let bell_superposition = h_2q.dot(&bell_init);
    println!("   After H ⊗ I: {}", to_dirac(&bell_superposition));

    let bell_state = gate_cnot().dot(&bell_superposition);
    println!("   Final Bell State: {}", to_dirac(&bell_state));

    let stats = test_measure(&bell_state, 100_000);
    let n_qubits = (bell_state.len() as f64).log2() as usize;

    let mut sorted_indices: Vec<_> = stats.keys().collect();
    sorted_indices.sort();

    for index in sorted_indices {
        let percentage = stats.get(&index).unwrap_or(&0.0);
        let bit_string = decode_measurement(*index, n_qubits);
        println!("    |{}>: {:.2}%", bit_string, percentage);
    }

    println!("\n   Analysis:");
    if stats.len() == 2 && stats.contains_key(&0) && stats.contains_key(&(bell_state.len() - 1)) {
        println!("   Perfect correlation detected: states |00> and |11> share the probability.");
        println!("   This confirms the non-local nature of the Bell State.");
    } else if stats.len() > 2 {
        println!("   State is in a multi-state superposition.");
    }
}

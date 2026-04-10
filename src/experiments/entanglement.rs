use crate::constants::{gate_cnot, gate_h, identity, q0};
use crate::formatting::to_dirac;
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

    println!("\n--- Statistical Verification (IMPORTANT NOTE) ---");
    println!("⚠️   CAUTION: The current measurement system is optimized for single qubits.");
    println!("    In a 2-qubit system, it treats state |00> as '0' and |11> as '1'.");
    println!("    The statistics below appear correct only because the Bell State lacks");
    println!("    intermediate states (|01> and |10>). Multi-qubit decoding will be");
    println!("    addressed in the next research session.\n");

    let (p1, p0) = test_measure(&bell_state, 100_000);
    println!("    Measurement Stats: |1> {:.2}% vs |0> {:.2}%\n", p1, p0);
}

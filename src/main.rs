use quantum_lab::constants::{gate_cnot, gate_h, gate_x, q0, q1};
use quantum_lab::formatting::to_dirac;
use quantum_lab::measurement::test_measure;
use quantum_lab::ops::tensor_product;

fn main() {
    println!("=== Quantum Lab ===\n");
    // --- Experiment 1: Single Qubit Logic ---
    println!("1.Single Qubit Operations:");

    let state_x = gate_x().dot(&q0());
    println!("   X|0> = {} (NOT gate)", to_dirac(&state_x));
    let state_h = gate_h().dot(&q0());
    println!("   H|0> = {} (Superposition)", to_dirac(&state_h));
    let state_xh = gate_h().dot(&state_x); // H(X|0>) = |->
    println!("   H(X|0>) = {} (Phase state |- >)", to_dirac(&state_xh));
    let (p1, p0) = test_measure(&state_xh, 100_000);
    println!("   Measurement Stats: |1> {:.2}% vs |0> {:.2}%\n", p1, p0);

    // --- Experiment 2: Multi-Qubit Tensor Product ---
    println!("2.Multi-Qubit Systems (Tensor Product):");
    let state_10 = tensor_product(&q1(), &q0());
    println!("   |1> ⊗ |0> = {}", to_dirac(&state_10));

    // --- Experiment 3: CNOT Verification ---
    println!("\n3.CNOT Gate Logic:");
    let cnot_res_10 = gate_cnot().dot(&state_10);
    println!("   CNOT|10> = {} (Flip expected)", to_dirac(&cnot_res_10));
    let state_01 = tensor_product(&q0(), &q1());
    let cnot_res_01 = gate_cnot().dot(&state_01);
    println!(
        "   CNOT|01> = {} (No flip expected)",
        to_dirac(&cnot_res_01)
    );
}

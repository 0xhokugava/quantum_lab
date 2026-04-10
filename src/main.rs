use quantum_lab::experiments::cnot_verification::cnot_verification;
use quantum_lab::experiments::entanglement::entanglement;
use quantum_lab::experiments::single_qubit::single_qubit;
use quantum_lab::experiments::tensor_product_example::tensor_product_example;

fn main() {
    println!("=== Quantum Lab ===\n");
    single_qubit();
    tensor_product_example();
    cnot_verification();
    entanglement();
}

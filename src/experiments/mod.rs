pub mod cnot_verification;
pub mod entanglement;
pub mod single_qubit;
pub mod tensor_product_example;

pub fn run_all() {
    single_qubit::run();
    tensor_product_example::run();
    cnot_verification::run();
    entanglement::run();
}

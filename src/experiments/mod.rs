mod cnot_inplace_verification;
pub mod cnot_verification;
pub mod entanglement;
mod inplace_verification;
pub mod single_qubit;
pub mod tensor_product_example;

pub fn run_all() {
    single_qubit::run();
    tensor_product_example::run();
    cnot_verification::run();
    entanglement::run();
    inplace_verification::run();
    cnot_inplace_verification::run();
}

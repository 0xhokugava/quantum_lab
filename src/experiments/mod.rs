mod cnot_inplace_verification;
mod cnot_verification;
mod entanglement;
mod inplace_verification;
mod single_qubit;
mod tensor_product_example;

pub fn run_all() {
    single_qubit::run();
    tensor_product_example::run();
    cnot_verification::run();
    entanglement::run();
    inplace_verification::run();
    cnot_inplace_verification::run();
}

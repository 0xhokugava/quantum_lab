use crate::constants::{q0, q1};
use crate::ops::tensor_product;
use crate::utils::to_dirac;

pub fn run() {
    println!("Multi-Qubit Systems (Tensor Product):\n");
    let state_10 = tensor_product(&q1(), &q0());
    println!("   |1> ⊗ |0> = {}", to_dirac(&state_10));
}

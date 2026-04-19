use crate::constants::{q0, q1};
use crate::utils::to_dirac;
use crate::ops::tensor_product;

pub fn run() {
    println!("2. Multi-Qubit Systems (Tensor Product):");
    let state_10 = tensor_product(&q1(), &q0());
    println!("   |1> ⊗ |0> = {}", to_dirac(&state_10));
}
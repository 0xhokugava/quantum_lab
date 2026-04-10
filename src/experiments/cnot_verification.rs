use crate::constants::{gate_cnot, q0, q1};
use crate::formatting::to_dirac;
use crate::ops::tensor_product;

pub fn cnot_verification() {
    println!("\n3. CNOT Gate Logic:");
    let state_10 = tensor_product(&q1(), &q0())
        .into_dimensionality::<ndarray::Ix1>()
        .expect("Should be a vector");

    let cnot_res_10 = gate_cnot().dot(&state_10);
    println!("   CNOT|10> = {} (Flip expected)", to_dirac(&cnot_res_10));
    let state_01 = tensor_product(&q0(), &q1());

    let state_01 = state_01
        .into_dimensionality::<ndarray::Ix1>()
        .expect("Should be a vector");

    let cnot_res_01 = gate_cnot().dot(&state_01);
    println!(
        "   CNOT|01> = {} (No flip expected)",
        to_dirac(&cnot_res_01)
    );
}

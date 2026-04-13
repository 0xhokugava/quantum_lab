use crate::constants::{gate_h, gate_x, q0};
use crate::formatting::to_dirac;
use crate::measurement::test_measure;

pub fn single_qubit() {
    println!("1. Single Qubit Operations:");

    let state_x = gate_x().dot(&q0());
    println!("   X|0> = {} (NOT gate)", to_dirac(&state_x));
    let state_h = gate_h().dot(&q0());
    println!("   H|0> = {} (Superposition)", to_dirac(&state_h));
    let state_xh = gate_h().dot(&state_x); // H(X|0>) = |->
    println!("   H(X|0>) = {} (Phase state |- >)", to_dirac(&state_xh));

    let stats = test_measure(&state_xh, 100_000);
    let p0 = stats.get(&0).unwrap_or(&0.0);
    let p1 = stats.get(&1).unwrap_or(&0.0);

    println!("   Measurement Stats: |1> {:.2}% vs |0> {:.2}%\n", p1, p0);
}

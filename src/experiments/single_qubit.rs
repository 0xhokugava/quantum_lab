use crate::constants::{gate_h, gate_x, gate_z, q0, q1};
use crate::measurement::test_measure;
use crate::utils::{approx_eq, to_dirac};

pub fn run() {
    println!("1. Single Qubit Operations:");

    let state_x = gate_x().dot(&q0());
    println!("   X|0> = {} (NOT gate)", to_dirac(&state_x));
    let state_h = gate_h().dot(&q0());
    println!("   H|0> = {} (Superposition)\n", to_dirac(&state_h));

    println!("   Identity Check: ZH == HX\n");

    // Scenario 1: Z(H|0>) vs H(X|0>)
    let zh_0 = gate_z().dot(&gate_h().dot(&q0()));
    let hx_0 = gate_h().dot(&gate_x().dot(&q0()));
    println!("   Z(H|0>) = {}", to_dirac(&zh_0));
    println!("   H(X|0>) = {}", to_dirac(&hx_0));

    // Scenario 2: Z(H|1>) vs H(X|1>)
    let zh_1 = gate_z().dot(&gate_h().dot(&q1()));
    let hx_1 = gate_h().dot(&gate_x().dot(&q1()));
    println!("   Z(H|1>) = {}", to_dirac(&zh_1));
    println!("   H(X|1>) = {}\n", to_dirac(&hx_1));

    if approx_eq(&zh_0, &hx_0, 1e-10) && approx_eq(&zh_1, &hx_1, 1e-10) {
        println!("   [SUCCESS] ZH == HX identity verified for basic states.\n");
    }

    let state_xh = gate_h().dot(&state_x);
    let stats = test_measure(&state_xh, 100_000);
    let p0 = stats.get(&0).unwrap_or(&0.0);
    let p1 = stats.get(&1).unwrap_or(&0.0);

    println!("   Measurement Stats: |1> {:.2}% vs |0> {:.2}%\n", p1, p0);
}

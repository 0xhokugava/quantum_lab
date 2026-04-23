use crate::constants::{gate_h, gate_s, gate_t, gate_x, gate_z, q0, q1};
use crate::measurement::test_measure;
use crate::utils::{approx_eq, to_dirac};

pub fn run() {
    println!("1. Single Qubit Operations:");

    // Basic Bit-flip (X) operation
    // Expect: |0> state to be transformed to |1>
    let state_x = gate_x().dot(&q0());
    println!("   X|0> = {} (NOT gate)", to_dirac(&state_x));
    // Hadamard (H) operation creates an equal superposition
    // Expect: 1/sqrt(2) * (|0> + |1>)
    let state_h = gate_h().dot(&q0());
    println!("   H|0> = {} (Superposition)\n", to_dirac(&state_h));

    println!("   Identity Check (on basis states): ZH == HX\n");
    // Scenario 1: Z(H|0>) vs H(X|0>)
    // Demonstrating that ZH and HX produce identical results on a basis states
    // (operator equivalence in this specific context)
    let zh_0 = gate_z().dot(&gate_h().dot(&q0()));
    let hx_0 = gate_h().dot(&gate_x().dot(&q0()));
    println!("   Z(H|0>) = {}", to_dirac(&zh_0));
    println!("   H(X|0>) = {}", to_dirac(&hx_0));

    // Scenario 2: Z(H|1>) vs H(X|1>)
    let zh_1 = gate_z().dot(&gate_h().dot(&q1()));
    let hx_1 = gate_h().dot(&gate_x().dot(&q1()));
    println!("   Z(H|1>) = {}", to_dirac(&zh_1));
    println!("   H(X|1>) = {}\n", to_dirac(&hx_1));

    // Verifying operator equivalence on basis states (ZH = HX)
    if approx_eq(&zh_0, &hx_0, 1e-10) && approx_eq(&zh_1, &hx_1, 1e-10) {
        println!("   [SUCCESS] ZH == HX identity verified for basic states.\n");
    }

    // Measuring the state resulting from H(X|0>)
    // Expect: Balanced 50/50 probability distribution
    let state_xh = gate_h().dot(&state_x);
    let stats_h = test_measure(&state_xh, 100_000);
    let p0 = stats_h.get(&0).unwrap_or(&0.0);
    let p1 = stats_h.get(&1).unwrap_or(&0.0);

    println!("   Measurement Stats: |1> {:.2}% vs |0> {:.2}%\n", p1, p0);
    println!("   Phase Gates Verification (Z, S, T)\n");

    // Initialize state |+> = H|0> to observe phase transformations
    let state_plus = gate_h().dot(&q0());
    println!("   Initial state |+> : {}", to_dirac(&state_plus));

    // 1. Apply Z gate (180-degree rotation)
    // Expect: |+> -> |-> (Phase flip on |1>)
    let after_z = gate_z().dot(&state_plus);
    println!("   After Z (π shift) : {}", to_dirac(&after_z));

    // 2. Apply S gate (90-degree rotation)
    // Expect: |1> amplitude becomes imaginary (i)
    let after_s = gate_s().dot(&state_plus);
    println!("   After S (π/2 shift): {}", to_dirac(&after_s));

    // 3. Apply T gate (45-degree rotation)
    // Expect: |1> amplitude becomes a complex mix (e^iπ/4)
    let after_t = gate_t().dot(&state_plus);
    println!("   After T (π/4 shift): {}\n", to_dirac(&after_t));

    // Verify that phase rotations are unitary and don't affect outcome probabilities
    let stats_t = test_measure(&after_t, 100_000);
    println!("   Verification: Phase gates preserve measurement probabilities.");
    let p0 = stats_t.get(&0).unwrap_or(&0.0);
    let p1 = stats_t.get(&1).unwrap_or(&0.0);
    println!("   Stats (after T): |0> {:.2}%, |1> {:.2}%\n", p0, p1);
}

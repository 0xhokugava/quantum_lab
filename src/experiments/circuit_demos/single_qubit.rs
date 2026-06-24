use crate::engine::measurement::test_measure;
use crate::engine::utils::{approx_eq, run_1q, to_dirac};

/// Runs a single-qubit gate demonstration using the high-level `Circuit` API.
///
/// This experiment covers:
/// - Basic X gate behavior: `X|0> = |1>`
/// - Hadamard superposition: `H|0> = (|0> + |1>) / sqrt(2)`
/// - Operator identity check on the basis states: `ZH == HX`
/// - Measurement statistics for balanced superposition
/// - Phase gate behavior for `Z`, `S`, and `T` applied to `|+>`
///
/// The goal is to demonstrate user-facing circuit construction rather than
/// direct matrix-vector multiplication.
pub fn run() {
    println!("Single Qubit Operations:\n");

    let shots = 100_000;
    let state_x = run_1q(|c| {
        c.x(0);
    });

    println!("   X|0> = {} (NOT gate)", to_dirac(&state_x));
    let state_h = run_1q(|c| {
        c.h(0);
    });

    println!("   H|0> = {} (Superposition)\n", to_dirac(&state_h));
    println!("   Identity Check (on basis states): ZH == HX\n");

    let zh_0 = run_1q(|c| {
        c.h(0).z(0);
    });

    let hx_0 = run_1q(|c| {
        c.x(0).h(0);
    });

    println!("   Z(H|0>) = {}", to_dirac(&zh_0));
    println!("   H(X|0>) = {}", to_dirac(&hx_0));

    let zh_1 = run_1q(|c| {
        c.x(0).h(0).z(0);
    });

    let hx_1 = run_1q(|c| {
        c.x(0).x(0).h(0);
    });

    println!("   Z(H|1>) = {}", to_dirac(&zh_1));
    println!("   H(X|1>) = {}\n", to_dirac(&hx_1));

    if approx_eq(&zh_0, &hx_0, 1e-10) && approx_eq(&zh_1, &hx_1, 1e-10) {
        println!("   [SUCCESS] ZH == HX identity verified for basic states.\n");
    }

    let state_xh = run_1q(|c| {
        c.x(0).h(0);
    });

    let stats_h = test_measure(&state_xh, shots);
    let p0 = stats_h.get(&0).unwrap_or(&0.0);
    let p1 = stats_h.get(&1).unwrap_or(&0.0);

    println!("   Measurement Stats: |1> {:.2}% vs |0> {:.2}%\n", p1, p0);
    println!("   Phase Gates Verification (Z, S, T)\n");
    println!("   Initial state |+> : {}", to_dirac(&state_h));

    let after_z = run_1q(|c| {
        c.h(0).z(0);
    });

    println!("   After Z (π shift) : {}", to_dirac(&after_z));

    let after_s = run_1q(|c| {
        c.h(0).s(0);
    });

    println!("   After S (π/2 shift): {}", to_dirac(&after_s));
    let after_t = run_1q(|c| {
        c.h(0).t(0);
    });

    println!("   After T (π/4 shift): {}\n", to_dirac(&after_t));
    let stats_t = test_measure(&after_t, shots);

    println!("   Verification: Phase gates preserve measurement probabilities.");

    let p0 = stats_t.get(&0).unwrap_or(&0.0);
    let p1 = stats_t.get(&1).unwrap_or(&0.0);

    println!("   Stats (after T): |0> {:.2}%, |1> {:.2}%\n", p0, p1);
}

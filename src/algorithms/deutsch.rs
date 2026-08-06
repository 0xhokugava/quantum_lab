use super::deutsch_helpers::{ANSWER, QUERY, classify_deutsch_result, query_probability};
use crate::circuit::core::Circuit;
use crate::engine::utils::to_dirac;
use crate::examples::catalog::deutsch_circuit;
use ndarray::ArrayD;
use num_complex::Complex64;
// Deutsch algorithm demo.
//
// This module implements the `n = 1` case of the Deutsch-Jozsa problem.
// The goal is to distinguish whether a one-bit Boolean function is
// constant or balanced using one oracle application.
//
// Qubit convention:
// - `QUERY` is the input qubit `x`.
// - `ANSWER` is the auxiliary qubit `y`.
// - The oracle has the form `Uf |x, y> = |x, y ⊕ f(x)>`.
//
// The final Hadamard on `QUERY` converts phase information into a readable result:
// - `query = 0` means the oracle is constant.
// - `query = 1` means the oracle is balanced.

#[derive(Clone, Copy, Debug)]
pub enum DeutschOracle {
    ConstantZero,
    ConstantOne,
    BalancedIdentity,
    BalancedNot,
}

const ORACLES: [DeutschOracle; 4] = [
    DeutschOracle::ConstantZero,
    DeutschOracle::ConstantOne,
    DeutschOracle::BalancedIdentity,
    DeutschOracle::BalancedNot,
];

pub struct DeutschResult {
    pub expected_class: &'static str,
    pub observed_class: &'static str,
    pub p_query_zero: f64,
    pub p_query_one: f64,
    pub state: ArrayD<Complex64>,
}

impl DeutschOracle {
    fn label(self) -> &'static str {
        match self {
            DeutschOracle::ConstantZero => "f(x) = 0",
            DeutschOracle::ConstantOne => "f(x) = 1",
            DeutschOracle::BalancedIdentity => "f(x) = x",
            DeutschOracle::BalancedNot => "f(x) = not x",
        }
    }

    fn expected_class(self) -> &'static str {
        match self {
            DeutschOracle::ConstantZero | DeutschOracle::ConstantOne => "constant",
            DeutschOracle::BalancedIdentity | DeutschOracle::BalancedNot => "balanced",
        }
    }

    pub(crate) fn apply(self, circuit: &mut Circuit) {
        match self {
            DeutschOracle::ConstantZero => {}
            DeutschOracle::ConstantOne => {
                circuit.x(ANSWER);
            }
            DeutschOracle::BalancedIdentity => {
                circuit.cnot(QUERY, ANSWER);
            }
            DeutschOracle::BalancedNot => {
                circuit.x(QUERY);
                circuit.cnot(QUERY, ANSWER);
                circuit.x(QUERY);
            }
        }
    }
}

fn deutsch_state(oracle: DeutschOracle) -> ArrayD<Complex64> {
    deutsch_circuit(oracle).run()
}

pub fn run_deutsch(oracle: DeutschOracle) -> DeutschResult {
    let state = deutsch_state(oracle);

    let p_query_zero = query_probability(&state, 0);
    let p_query_one = query_probability(&state, 1);
    let observed_class = classify_deutsch_result(&state);

    DeutschResult {
        expected_class: oracle.expected_class(),
        observed_class,
        p_query_zero,
        p_query_one,
        state,
    }
}

pub fn run_deutsch_demo() {
    println!("\nDeutsch algorithm (n = 1 Deutsch-Jozsa):\n");

    for oracle in ORACLES {
        let result = run_deutsch(oracle);

        println!("   Oracle: {}", oracle.label());
        println!("   Expected class: {}", result.expected_class);
        println!("   Observed class: {}", result.observed_class);
        println!(
            "   Query probabilities: P(0) = {:.6}, P(1) = {:.6}",
            result.p_query_zero, result.p_query_one
        );
        println!("   Final state:");
        println!("   {}", to_dirac(&result.state));
        println!();
    }
}

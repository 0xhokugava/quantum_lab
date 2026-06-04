use crate::algorithms::deutsch_helpers::{
    ANSWER, classify_deutsch_jozsa_result, query_register_zero_probability,
};
use crate::circuit::Circuit;
use crate::utils::to_dirac;
use ndarray::ArrayD;
use num_complex::Complex64;
use std::ops::RangeInclusive;

// Deutsch-Jozsa algorithm demo.
//
// This module implements the Deutsch-Jozsa algorithm for Boolean oracles that
// are guaranteed to be either constant or balanced. The circuit uses one answer
// qubit and one or more query qubits.
//
// Qubit layout:
// - ANSWER is qubit 0
// - query qubits are 1..=num_query_qubits
//
// The oracle applies Uf |x, y> = |x, y xor f(x)>.
// After the final Hadamard's on the query register, the result is classified by
// checking the probability of the query register being |00...0>.
//
// Expected behavior:
// - constant oracle -> query register returns |00...0>
// - balanced oracle -> query register does not return |00...0>

#[derive(Debug, Clone, Copy)]
pub enum DeutschJozsaOracle {
    ConstantZero,
    ConstantOne,
    BalancedSingleBit { query: usize },
    BalancedParity,
}
pub struct DeutschJozsaResult {
    pub expected_class: &'static str,
    pub observed_class: &'static str,
    pub p_query_zero: f64,
    pub state: ArrayD<Complex64>,
}

impl DeutschJozsaOracle {
    fn apply(self, circuit: &mut Circuit, num_query_qubits: usize) {
        match self {
            DeutschJozsaOracle::ConstantZero => {}
            DeutschJozsaOracle::ConstantOne => {
                circuit.x(ANSWER);
            }
            DeutschJozsaOracle::BalancedSingleBit { query } => {
                assert!(
                    (1..=num_query_qubits).contains(&query),
                    "query must be in 1..={}",
                    num_query_qubits
                );
                circuit.cnot(query, ANSWER);
            }
            DeutschJozsaOracle::BalancedParity => {
                for query in query_qubits(num_query_qubits) {
                    circuit.cnot(query, ANSWER);
                }
            }
        }
    }
}

impl DeutschJozsaOracle {
    fn label(self) -> &'static str {
        match self {
            DeutschJozsaOracle::ConstantZero => "f(x) = 0",
            DeutschJozsaOracle::ConstantOne => "f(x) = 1",
            DeutschJozsaOracle::BalancedSingleBit { .. } => "f(x) = selected query bit",
            DeutschJozsaOracle::BalancedParity => "f(x) = parity of query register",
        }
    }
    fn expected_class(self) -> &'static str {
        match self {
            DeutschJozsaOracle::ConstantZero | DeutschJozsaOracle::ConstantOne => "constant",
            DeutschJozsaOracle::BalancedSingleBit { .. } | DeutschJozsaOracle::BalancedParity => {
                "balanced"
            }
        }
    }
}

fn query_qubits(num_query_qubits: usize) -> RangeInclusive<usize> {
    1..=num_query_qubits
}

fn apply_h_to_queries(circuit: &mut Circuit, num_query_qubits: usize) {
    for query in query_qubits(num_query_qubits) {
        circuit.h(query);
    }
}

fn deutsch_jozsa_state(num_query_qubits: usize, oracle: DeutschJozsaOracle) -> ArrayD<Complex64> {
    assert!(
        num_query_qubits > 0,
        "Deutsch-Jozsa requires at least one query qubit"
    );
    let total_qubits = num_query_qubits + 1;
    let mut circuit = Circuit::new(total_qubits);
    circuit.x(ANSWER);

    apply_h_to_queries(&mut circuit, num_query_qubits);

    circuit.h(ANSWER);

    oracle.apply(&mut circuit, num_query_qubits);

    apply_h_to_queries(&mut circuit, num_query_qubits);

    circuit.run()
}

pub fn run_deutsch_jozsa(
    num_query_qubits: usize,
    oracle: DeutschJozsaOracle,
) -> DeutschJozsaResult {
    let state = deutsch_jozsa_state(num_query_qubits, oracle);

    let p_query_zero = query_register_zero_probability(&state, num_query_qubits);
    let observed_class = classify_deutsch_jozsa_result(&state, num_query_qubits);

    DeutschJozsaResult {
        expected_class: oracle.expected_class(),
        observed_class,
        p_query_zero,
        state,
    }
}

pub fn run_deutsch_jozsa_demo() {
    let num_query_qubits = 3;

    let oracles = [
        DeutschJozsaOracle::ConstantZero,
        DeutschJozsaOracle::ConstantOne,
        DeutschJozsaOracle::BalancedSingleBit { query: 1 },
        DeutschJozsaOracle::BalancedParity,
    ];

    println!("8. Deutsch-Jozsa algorithm\n");

    for oracle in oracles {
        let result = run_deutsch_jozsa(num_query_qubits, oracle);

        println!("   Oracle: {}", oracle.label());
        println!("   Query qubits: {}", num_query_qubits);
        println!("   Expected class: {}", result.expected_class);
        println!("   Observed class: {}", result.observed_class);
        println!(
            "   P(query register = |00...0>): {:.6}",
            result.p_query_zero
        );
        println!("   Final state:");
        println!("   {}", to_dirac(&result.state));
        println!();
    }
}

use eigenon::algorithms::deutsch_jozsa::{DeutschJozsaOracle, run_deutsch_jozsa};

const EPSILON: f64 = 1e-9;

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < EPSILON,
        "expected {}, got {}",
        expected,
        actual
    );
}

#[test]
fn deutsch_jozsa_classifies_constant_oracles() {
    let num_query_qubits = 3;

    let oracles = [
        DeutschJozsaOracle::ConstantZero,
        DeutschJozsaOracle::ConstantOne,
    ];

    for oracle in oracles {
        let result = run_deutsch_jozsa(num_query_qubits, oracle);

        assert_eq!(result.expected_class, "constant");
        assert_eq!(result.observed_class, "constant");
        assert_close(result.p_query_zero, 1.0);
    }
}

#[test]
fn deutsch_jozsa_classifies_balanced_oracles() {
    let num_query_qubits = 3;

    let oracles = [
        DeutschJozsaOracle::BalancedSingleBit { query: 1 },
        DeutschJozsaOracle::BalancedSingleBit { query: 2 },
        DeutschJozsaOracle::BalancedSingleBit { query: 3 },
        DeutschJozsaOracle::BalancedParity,
    ];

    for oracle in oracles {
        let result = run_deutsch_jozsa(num_query_qubits, oracle);

        assert_eq!(result.expected_class, "balanced");
        assert_eq!(result.observed_class, "balanced");
        assert_close(result.p_query_zero, 0.0);
    }
}

use eigenon::algorithms::deutsch::{DeutschOracle, run_deutsch};

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
fn constant_oracles_are_classified_as_constant() {
    let oracles = [DeutschOracle::ConstantZero, DeutschOracle::ConstantOne];

    for oracle in oracles {
        let result = run_deutsch(oracle);

        assert_eq!(result.expected_class, "constant");
        assert_eq!(result.observed_class, "constant");
        assert_close(result.p_query_zero, 1.0);
        assert_close(result.p_query_one, 0.0);
    }
}

#[test]
fn balanced_oracles_are_classified_as_balanced() {
    let oracles = [DeutschOracle::BalancedIdentity, DeutschOracle::BalancedNot];

    for oracle in oracles {
        let result = run_deutsch(oracle);

        assert_eq!(result.expected_class, "balanced");
        assert_eq!(result.observed_class, "balanced");
        assert_close(result.p_query_zero, 0.0);
        assert_close(result.p_query_one, 1.0);
    }
}

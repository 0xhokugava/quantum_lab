use ndarray::Array1;
use rand::*;

/// Performs a quantum measurement on a single qubit state vector.
/// Collapses the superposition into a basis state |0> or |1> based on Born's rule.
/// TODO NOTE: The current implementation is optimized for single-qubit measurement.
pub fn measure(arr: &Array1<f64>) -> i32 {
    let mut probabilities: Array1<f64> = Array1::zeros(arr.len());
    for i in 0..arr.len() {
        // Probability is the square of the amplitude (Born's rule)
        probabilities[i] = arr[i].powi(2);
    }
    let mut rng = rng();
    let dice = rng.random_range(0.0..=1.0);
    if dice < probabilities[0] { 0 } else { 1 }
}

/// Runs multiple measurement simulations (shots) to gather statistics.
/// Returns a tuple containing the percentage of outcomes for |1> and |0>
pub fn test_measure(state: &Array1<f64>, shots: usize) -> (f64, f64) {
    let mut one_count: usize = 0;

    for _ in 0..shots {
        if measure(state) == 1 {
            one_count += 1;
        }
    }
    let zero_count: usize = shots - one_count;
    // Calculate statistical distribution in percentages
    let p_one = (one_count as f64 / shots as f64) * 100.0;
    let p_zero = (zero_count as f64 / shots as f64) * 100.0;

    (p_one, p_zero)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{q0, q1};
    use ndarray::array;
    use std::f64::consts::FRAC_1_SQRT_2;

    #[test]
    fn test_measure_deterministic() {
        // |0> must always collapse to 0
        for _ in 0..1000 {
            assert_eq!(measure(&q0()), 0);
        }

        // |1> must always collapse to 1
        for _ in 0..1000 {
            assert_eq!(measure(&q1()), 1);
        }
    }

    #[test]
    fn test_measure_statistics() {
        let h_state = array![FRAC_1_SQRT_2, FRAC_1_SQRT_2];
        let shots = 1000;
        let (p1, p0) = test_measure(&h_state, shots);

        // For Hadamard expectations roughly 50/50
        // Check if it's within ±10% to avoid rare random fails
        assert!(p1 > 40.0 && p1 < 60.0);
        assert!(p0 > 40.0 && p0 < 60.0);

        // Sum must be exactly 100% or very close
        assert!((p1 + p0 - 100.0).abs() < 1e-9);
    }
}

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

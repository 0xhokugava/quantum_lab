use ndarray::Array1;
use rand::*;
use std::collections::HashMap;
use num_complex::Complex64;

/// Performs a quantum measurement on a state vector of any size.
/// Collapses the superposition into a basis state based on Born's rule (|ψ|²).
/// Uses cumulative probability to select an outcome from the full distribution.
pub fn measure(arr: &Array1<Complex64>) -> usize {
    let mut rng = rng();
    let dice = rng.random_range(0.0..=1.0);

    let mut cumulative_probability = 0.0;

    for (index, amplitude) in arr.iter().enumerate() {
        cumulative_probability += amplitude.norm_sqr();
        if dice < cumulative_probability {
            return index;
        }
    }

    arr.len() - 1
}

/// Runs multiple measurement simulations (shots) to gather statistics.
/// Returns a HashMap containing the probability distribution across all detected states.
pub fn test_measure(state: &Array1<Complex64>, shots: usize) -> HashMap<usize, f64> {
    let mut counts = HashMap::new();

    for _ in 0..shots {
        let res = measure(state);
        *counts.entry(res).or_insert(0.0) += 1.0;
    }

    for count in counts.values_mut() {
        *count = (*count / shots as f64) * 100.0;
    }

    counts
}

use ndarray::ArrayD;
use num_complex::Complex64;

// Deutsch algorithm qubit convention:
// query - input qubit x
// answer - auxiliary qubit y
//
// The simulator uses little-endian-style indexing here:
// basis index bits are inspected with (basis_index >> qubit_index) & 1.
pub(crate) const QUERY: usize = 1;
pub(crate) const ANSWER: usize = 0;

pub(crate) fn query_probability(state: &ArrayD<Complex64>, query_value: usize) -> f64 {
    state
        .iter()
        .enumerate()
        .filter(|(basis_index, _)| ((basis_index >> QUERY) & 1) == query_value)
        .map(|(_, amplitude)| amplitude.norm_sqr())
        .sum()
}

pub(crate) fn classify_deutsch_result(state: &ArrayD<Complex64>) -> &'static str {
    let p_query_zero = query_probability(state, 0);
    let p_query_one = query_probability(state, 1);

    if p_query_zero > p_query_one {
        "constant"
    } else {
        "balanced"
    }
}

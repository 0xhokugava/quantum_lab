use ndarray::ArrayD;
use num_complex::Complex64;

// Deutsch algorithm qubit convention:
// query - input qubit x
// answer - auxiliary qubit y
//
// The simulator uses little-endian qubit indexing:
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

/// Computes P(query register = |00...0>) while ignoring the ANSWER qubit.
/// Layout:
/// - bit 0 is ANSWER
/// - bits 1..=num_query_qubits are QUERY bits
pub(crate) fn query_register_zero_probability(
    state: &ArrayD<Complex64>,
    num_query_qubits: usize,
) -> f64 {
    let query_mask = ((1usize << num_query_qubits) - 1) << 1;

    state
        .iter()
        .enumerate()
        .filter(|(basis_index, _)| (basis_index & query_mask) == 0)
        .map(|(_, amplitude)| amplitude.norm_sqr())
        .sum()
}

/// Deutsch-Jozsa classification rule:
/// constant oracle -> query register ends in |00...0>
/// balanced oracle -> query register never ends in |00...0>
pub(crate) fn classify_deutsch_jozsa_result(
    state: &ArrayD<Complex64>,
    num_query_qubits: usize,
) -> &'static str {
    let p_zero = query_register_zero_probability(state, num_query_qubits);
    if p_zero > 0.5 { "constant" } else { "balanced" }
}

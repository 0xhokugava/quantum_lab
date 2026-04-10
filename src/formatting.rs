use ndarray::{Array, Dimension};

/// Formats a quantum state (represented as a vector or dynamic array) into Dirac notation.
/// Filters out near-zero amplitudes and represents indices in binary format.
///
/// Developed by Paul Dirac in 1939 to simplify quantum state representation.
pub fn to_dirac<D: Dimension>(state: &Array<f64, D>) -> String {
    let state = state.view().into_dyn();
    // Determine the number of qubits based on the vector length (2^n)
    let n_qubits = (state.len() as f64).log2() as usize;

    state
        .iter()
        .enumerate()
        .filter(|(_, val)| val.abs() > 1e-6)
        .map(|(i, &val)| {
            let sign = if val > 0.0 { "+" } else { "-" };
            // Format index 'i' as a binary string of length 'n_qubits'
            format!(
                "{}{:.3}|{:0width$b}>",
                sign,
                val.abs(),
                i,
                width = n_qubits
            )
        })
        .collect::<Vec<String>>()
        .join("")
}

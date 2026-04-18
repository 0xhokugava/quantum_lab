use ndarray::{Array, Dimension};
use num_complex::Complex64;

/// Formats a quantum state (represented as a vector or dynamic array) into Dirac notation.
/// Filters out near-zero amplitudes and represents indices in binary format.
///
/// Developed by Paul Dirac in 1939 to simplify quantum state representation.
pub fn to_dirac<D: Dimension>(state: &Array<Complex64, D>) -> String {
    let state = state.view().into_dyn();
    // Determine the number of qubits based on the vector length (2^n)
    let n_qubits = (state.len() as f64).log2() as usize;

    state
        .iter()
        .enumerate()
        .filter(|(_, val)| val.norm() > 1e-6)
        .map(|(i, &val)| {
            format!(
                "({:.3} + {:.3}i)|{:0width$b}>",
                val.re,
                val.im,
                i,
                width = n_qubits
            )
        })
        .collect::<Vec<String>>()
        .join("")
}

/// Converts a state index into a human-readable binary string (Dirac notation).
pub fn decode_measurement(index: usize, n_qubits: usize) -> String {
    format!("{:0width$b}", index, width = n_qubits)
}

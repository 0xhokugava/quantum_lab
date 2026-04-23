use ndarray::{Array, Array1, ArrayD, Dimension};
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

/// Compares two quantum state vectors element-wise using an epsilon tolerance.
///
/// Returns true if the difference between corresponding amplitudes
/// is smaller than the specified threshold (`eps`) for all elements.
///
/// This is necessary due to floating-point precision errors in numerical computations.
///
/// Note:
/// - This comparison does NOT account for global phase differences.
/// - this function will consider two states that differ only by a global phase (e.g., ψ and -ψ) different.
/// - Suitable for simple validation, but not for full physical equivalence checks.
pub fn approx_eq(a: &Array1<Complex64>, b: &Array1<Complex64>, eps: f64) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).all(|(x, y)| (x - y).norm() < eps)
}

/// Creates an initial state vector |0...0> for N qubits.
/// The vector has 2^n elements, with the first element set to 1.
pub fn q0_n(n: usize) -> ArrayD<Complex64> {
    let size = 1 << n;
    let mut state = Array1::zeros(size);
    state[0] = Complex64::new(1.0, 0.0);
    state.into_dyn()
}

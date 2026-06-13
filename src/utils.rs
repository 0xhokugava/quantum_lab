use crate::circuit::Circuit;
use ndarray::{Array, Array1, Array2, ArrayD, Dimension};
use num_complex::Complex64;

/// Formats a quantum state (represented as a vector or dynamic array) into Dirac notation.
/// Filters out near-zero amplitudes and represents indices in binary format.
pub fn to_dirac<D: Dimension>(state: &Array<Complex64, D>) -> String {
    let state = state.view().into_dyn();
    let n_qubits = (state.len() as f64).log2() as usize;

    let terms = state
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
        .collect::<Vec<String>>();

    if terms.is_empty() {
        "0".to_string()
    } else {
        terms.join(" + ")
    }
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
/// - This function will consider two states that differ only by a global phase
///   (e.g., ψ and -ψ) different.
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

/// Asserts that two quantum state vectors are approximately equal.
///
/// This function compares two state vectors element-wise using a small
/// numerical tolerance to account for floating-point precision errors.
/// It assumes both inputs represent flattened quantum states (1D),
/// even if stored as `ArrayD`.
///
/// Panics if:
/// - The vectors have different lengths
/// - Any pair of amplitudes differs beyond the allowed tolerance
///
/// This is used as a validation step to ensure the correctness of
/// optimized (in-place) implementations against matrix-based baselines.
pub fn assert_states_close(a: &ArrayD<Complex64>, b: &ArrayD<Complex64>) {
    assert_eq!(a.len(), b.len());
    for (i, (x, y)) in a.iter().zip(b.iter()).enumerate() {
        assert!(
            (x - y).norm() < 1e-15,
            "Mismatch at index {}: expected {}, got {}",
            i,
            y,
            x
        );
    }
}

/// Converts a real `f64` value into a `Complex64` number with zero imaginary part.
///
/// This is a small helper for constructing real-valued matrices in tests,
/// where coefficients are often written as plain real numbers.
pub fn to_c64(re: f64) -> Complex64 {
    Complex64::new(re, 0.0)
}

/// Builds the full `2^n × 2^n` matrix representation of a local k-qubit gate.
///
/// This helper is intended for tests only. It constructs the dense global operator
/// corresponding to applying `gate` on the qubits listed in `targets`, while all
/// other qubits are left unchanged.
///
/// The local gate must have the shape `2^k × 2^k`, where `k = targets.len()`.
/// The order of `targets` defines how local gate indices are mapped to global
/// qubit positions.
///
/// This function is useful as a correctness baseline for comparing the optimized
/// in-place implementation against a dense matrix-based result.
pub fn build_full_operator(
    gate: &Array2<Complex64>,
    targets: &[usize],
    n: usize,
) -> Array2<Complex64> {
    let size = 1 << n;
    let mut full = Array2::<Complex64>::zeros((size, size));

    let k = targets.len();
    let dim = 1 << k;

    for col in 0..size {
        let mut local_col = 0;
        for (pos, &t) in targets.iter().enumerate() {
            if (col >> t) & 1 == 1 {
                local_col |= 1 << (k - 1 - pos);
            }
        }

        let mut base = col;
        for &t in targets {
            base &= !(1 << t);
        }

        for local_row in 0..dim {
            let mut row = base;
            for (pos, &t) in targets.iter().enumerate() {
                if (local_row >> (k - 1 - pos)) & 1 == 1 {
                    row |= 1 << t;
                }
            }
            full[[row, col]] = gate[[local_row, local_col]];
        }
    }
    full
}

/// Converts a dynamically shaped quantum state into a 1D state vector.
///
/// This helper is useful when code paths return `ArrayD<Complex64>`,
/// while measurement or formatting utilities expect `Array1<Complex64>`.
pub fn to_1d(state: ArrayD<Complex64>) -> Array1<Complex64> {
    state
        .into_dimensionality::<ndarray::Ix1>()
        .expect("Circuit output must be a 1D state vector")
}

/// Builds and runs a single-qubit circuit from a small closure.
///
/// This helper keeps single-qubit experiments concise by avoiding repeated
/// `Circuit::new(1)` and `run()` boilerplate for independent scenarios.
pub fn run_1q(build: impl FnOnce(&mut Circuit)) -> Array1<Complex64> {
    run_circuit(1, build)
}

/// Builds and runs a circuit with the given number of qubits.
///
/// This helper keeps experiments concise by avoiding repeated
/// `Circuit::new(n)` and `run()` boilerplate for independent scenarios.
pub fn run_circuit(n_qubits: usize, build: impl FnOnce(&mut Circuit)) -> Array1<Complex64> {
    let mut circuit = Circuit::new(n_qubits);
    build(&mut circuit);
    to_1d(circuit.run())
}

use ndarray::{Array1, Array2, array};
use num_complex::Complex64;
use std::f64::consts::FRAC_1_SQRT_2;

// Computational Basis States

/// Ground state |0⟩.
/// Represented as a column vector [1.0, 0.0].
pub fn q0() -> Array1<Complex64> {
    array![1.0.into(), 0.0.into()]
}

/// Excited state |1⟩.
/// Represented as a column vector [0.0, 1.0].
pub fn q1() -> Array1<Complex64> {
    array![0.0.into(), 1.0.into()]
}

// Single-Qubit Gates

/// Hadamard gate (H).
/// Creates a balanced superposition state.
/// Maps |0⟩ to (|0⟩ + |1⟩)/√2 and |1⟩ to (|0⟩ - |1⟩)/√2.
pub fn gate_h() -> Array2<Complex64> {
    let scale = Complex64::new(FRAC_1_SQRT_2, 0.0);
    array![[1.0.into(), 1.0.into()], [1.0.into(), (-1.0).into()]] * scale
}

/// Pauli-X gate (Quantum NOT).
/// Flips the qubit state: |0⟩ ↔ |1⟩.
pub fn gate_x() -> Array2<Complex64> {
    array![[0.0.into(), 1.0.into()], [1.0.into(), 0.0.into()]]
}

// Multi-Qubit Gates

/// Controlled-NOT (CNOT) gate.
/// Inverts the target (second) qubit if the control (first) qubit is |1⟩.
/// Operates on a 4-dimensional state vector (2 qubits).
pub fn gate_cnot() -> Array2<Complex64> {
    array![
        [1.0.into(), 0.0.into(), 0.0.into(), 0.0.into()],
        [0.0.into(), 1.0.into(), 0.0.into(), 0.0.into()],
        [0.0.into(), 0.0.into(), 0.0.into(), 1.0.into()],
        [0.0.into(), 0.0.into(), 1.0.into(), 0.0.into()],
    ]
}

/// Returns a 2x2 Identity matrix (I).
/// In quantum mechanics, an identity matrix represents a no-op gate.
/// It is essential for multi-qubit systems when we want to apply a gate
/// to one qubit while leaving another qubit unchanged (e.g., H ⊗ I).
pub fn identity() -> Array2<Complex64> {
    array![[1.0.into(), 0.0.into()], [0.0.into(), 1.0.into()]]
}

/// Pauli-Z gate (phase flip).
///
/// Applies a phase shift of π to the |1⟩ state while leaving |0⟩ unchanged:
/// |0⟩ → |0⟩
/// |1⟩ → -|1⟩
///
/// This gate does NOT change measurement probabilities
/// but modifies the relative phase of the quantum state,
/// which affects interference in later operations.
pub fn gate_z() -> Array2<Complex64> {
    array![[1.0.into(), 0.0.into()], [0.0.into(), (-1.0).into()]]
}

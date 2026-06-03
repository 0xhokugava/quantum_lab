use ndarray::{Array1, Array2, array};
use num_complex::Complex64;
use std::f64::consts::{FRAC_1_SQRT_2, PI};

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

// Clifford Gates

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

/// Pauli-Y gate.
///
/// A combination of a bit-flip (X) and a phase-flip (Z).
/// Maps |0⟩ → i|1⟩ and |1⟩ → -i|0⟩.
///
/// Mathematically, Y = iXZ. It represents a rotation of π radians
/// around the Y-axis of the Bloch Sphere.
pub fn gate_y() -> Array2<Complex64> {
    let i = Complex64::i();
    array![[0.0.into(), -i], [i, 0.0.into()]]
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

/// S gate (Phase gate).
///
/// Applies a phase shift of π/2 (90°) to the |1⟩ state.
/// Also known as the √Z gate, as applying it twice results in a Z gate.
///
/// Maps:
/// |0⟩ → |0⟩
/// |1⟩ → i|1⟩
pub fn gate_s() -> Array2<Complex64> {
    let i = Complex64::i();
    array![[1.0.into(), 0.0.into()], [0.0.into(), i]]
}

// Non-Clifford Gates

/// T gate (π/8 gate).
///
/// Applies a phase shift of π/4 (45°) to the |1⟩ state.
/// It is the fourth root of the Z gate (√S).
///
/// The T gate is crucial for universal quantum computation,
/// as it allows the construction of non-Clifford gates.
///
/// Maps:
/// |0⟩ → |0⟩
/// |1⟩ → e^(iπ/4)|1⟩
pub fn gate_t() -> Array2<Complex64> {
    let phase = Complex64::from_polar(1.0, PI / 4.0);
    array![[1.0.into(), 0.0.into()], [0.0.into(), phase]]
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

/// Controlled-Z (CZ) gate.
pub fn gate_cz() -> Array2<Complex64> {
    array![
        [1.0.into(), 0.0.into(), 0.0.into(), 0.0.into()],
        [0.0.into(), 1.0.into(), 0.0.into(), 0.0.into()],
        [0.0.into(), 0.0.into(), 1.0.into(), 0.0.into()],
        [0.0.into(), 0.0.into(), 0.0.into(), (-1.0).into()],
    ]
}

/// Returns a 2x2 Identity matrix (I).
/// In quantum mechanics, an identity matrix represents a no-op gate.
/// It is essential for multi-qubit systems when we want to apply a gate
/// to one qubit while leaving another qubit unchanged (e.g., H ⊗ I).
pub fn identity() -> Array2<Complex64> {
    array![[1.0.into(), 0.0.into()], [0.0.into(), 1.0.into()]]
}

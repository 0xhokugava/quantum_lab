use ndarray::{Array1, Array2, array};

// Computational Basis States

/// Ground state |0⟩.
/// Represented as a column vector [1.0, 0.0].
pub fn q0() -> Array1<f64> {
    array![1.0, 0.0]
}

/// Excited state |1⟩.
/// Represented as a column vector [0.0, 1.0].
pub fn q1() -> Array1<f64> {
    array![0.0, 1.0]
}

// Single-Qubit Gates

/// Hadamard gate (H).
/// Creates a balanced superposition state.
/// Maps |0⟩ to (|0⟩ + |1⟩)/√2 and |1⟩ to (|0⟩ - |1⟩)/√2.
pub fn gate_h() -> Array2<f64> {
    let h_factor = 2.0_f64.sqrt().recip();
    array![[1.0, 1.0], [1.0, -1.0]] * h_factor
}

/// Pauli-X gate (Quantum NOT).
/// Flips the qubit state: |0⟩ ↔ |1⟩.
pub fn gate_x() -> Array2<f64> {
    array![[0.0, 1.0], [1.0, 0.0]]
}

// Multi-Qubit Gates

/// Controlled-NOT (CNOT) gate.
/// Inverts the target (second) qubit if the control (first) qubit is |1⟩.
/// Operates on a 4-dimensional state vector (2 qubits).
pub fn gate_cnot() -> Array2<f64> {
    array![
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 1.0], // Flip occurs when the control qubit is in state |1⟩
        [0.0, 0.0, 1.0, 0.0]
    ]
}

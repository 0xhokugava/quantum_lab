use ndarray::{Array1, Array2, array};
use std::f64::consts::FRAC_1_SQRT_2;

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
    array![[1.0, 1.0], [1.0, -1.0]] * FRAC_1_SQRT_2
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

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_gate_x_logic() {
        // X|0> = |1>
        let result = gate_x().dot(&q0());
        assert_eq!(result, &q1());
    }

    #[test]
    fn test_gate_h_reversibility() {
        // H(H|0>) = |0>
        let once = gate_h().dot(&q0());
        let twice = gate_h().dot(&once);

        // Use tolerance check for float precision
        for (a, b) in twice.iter().zip(q0().iter()) {
            assert!((a - b).abs() < 1e-10);
        }
    }

    #[test]
    fn test_gate_h_is_unitary() {
        // Norm of H|0> must be 1.0
        let result = gate_h().dot(&q0());
        let norm: f64 = result.mapv(|x| x.powi(2)).sum();
        assert!((norm - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_cnot_logic() {
        // CNOT|10> = |11>
        // |10> is index 2: [0, 0, 1, 0]
        let input = array![0.0, 0.0, 1.0, 0.0];
        let output = gate_cnot().dot(&input);
        assert_eq!(output, array![0.0, 0.0, 0.0, 1.0]);

        // CNOT|01> = |01> (No change since control is 0)
        let input_01 = array![0.0, 1.0, 0.0, 0.0];
        let output_01 = gate_cnot().dot(&input_01);
        assert_eq!(output_01, input_01);
    }
}

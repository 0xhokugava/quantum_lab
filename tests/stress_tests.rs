use ndarray::{Array1, Array2};
use num_complex::Complex64;

#[test]
fn stress_large_state() {
    // number of qubits
    let n = 14;

    // Compute the size of the state vector: 2^n (number of basis states)
    let size = 1 << n;

    // Initialize a state vector of length 2^n where each amplitude is (1 + 0i)
    // NOTE: this is NOT a normalized quantum state (a sum of |ψ|^2 = size, not 1)
    let state = Array1::from(vec![Complex64::new(1.0, 0.0); size]);

    // identity gate (worst-case dense multiply)
    let gate = Array2::from_shape_fn((size, size), |_| Complex64::new(1.0, 0.0));
    let result = gate.dot(&state);
    assert_eq!(result.len(), size);
}

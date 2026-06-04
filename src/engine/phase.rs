use ndarray::ArrayD;
use num_complex::Complex64;

/// Applies a phase factor to all basis states matching a bit pattern under a mask.
///
/// Only bits selected by `mask` are compared. A basis state matches when:
/// `(basis_index & mask) == pattern`.
///
/// This is a low-level state-vector primitive for phase marking. If `phase` has
/// unit magnitude, measurement probabilities are unchanged.
pub fn apply_phase_on_basis_match(
    state: &mut ArrayD<Complex64>,
    num_qubits: usize,
    mask: usize,
    pattern: usize,
    phase: Complex64,
) {
    assert_eq!(state.len(), 1usize << num_qubits);
    assert_eq!(pattern & !mask, 0, "pattern must be inside mask");

    for basis_index in 0..state.len() {
        if (basis_index & mask) == pattern {
            state[basis_index] *= phase;
        }
    }
}

/// Builds a bit mask from a list of qubit indices.
///
/// Each selected qubit sets the corresponding bit in the returned mask.
/// Qubit indices follow the simulator convention: qubit 0 is the least
/// significant bit of the basis index.
pub fn qubit_mask(qubits: &[usize], num_qubits: usize) -> usize {
    let mut mask = 0usize;

    for &qubit in qubits {
        assert!(qubit < num_qubits);
        mask |= 1usize << qubit;
    }

    mask
}

/// Applies a multi-controlled Z phase flip in-place.
///
/// The phase is flipped only for basis states where all selected `qubits`
/// are set to 1. For example, with qubits `[0, 1, 2]`, only `|111>`
/// receives a `-1` phase.
pub fn apply_mcz_in_place(state: &mut ArrayD<Complex64>, num_qubits: usize, qubits: &[usize]) {
    assert!(!qubits.is_empty());

    let mask = qubit_mask(qubits, num_qubits);
    let phase = Complex64::new(-1.0, 0.0);

    apply_phase_on_basis_match(state, num_qubits, mask, mask, phase);
}

/// Applies an in-place phase oracle for a single marked basis state.
///
/// The target basis state receives a `-1` phase, while all other amplitudes
/// remain unchanged. This is the state-vector form of marking one solution
/// state for amplitude amplification.
pub fn apply_phase_oracle_in_place(
    state: &mut ArrayD<Complex64>,
    num_qubits: usize,
    target_index: usize,
) {
    assert!(target_index < (1usize << num_qubits));

    let full_mask = (1usize << num_qubits) - 1;
    let phase = Complex64::new(-1.0, 0.0);

    apply_phase_on_basis_match(state, num_qubits, full_mask, target_index, phase);
}

/// Applies inversion about the mean to all amplitudes in-place.
/// Each amplitude is reflected around the average amplitude:
/// `new_amplitude = 2 * mean - old_amplitude`.
///
/// This operation can be used as an amplitude amplification primitive.
pub fn apply_diffusion_in_place(state: &mut ArrayD<Complex64>) {
    let amplitude_sum: Complex64 = state.iter().sum();
    let mean = amplitude_sum / Complex64::new(state.len() as f64, 0.0);

    for amplitude in state.iter_mut() {
        *amplitude = (Complex64::new(2.0, 0.0) * mean) - *amplitude;
    }
}

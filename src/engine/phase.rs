use ndarray::ArrayD;
use num_complex::Complex64;

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

pub fn qubit_mask(qubits: &[usize], num_qubits: usize) -> usize {
    let mut mask = 0usize;

    for &qubit in qubits {
        assert!(qubit < num_qubits);
        mask |= 1usize << qubit;
    }

    mask
}

pub fn apply_mcz_in_place(state: &mut ArrayD<Complex64>, num_qubits: usize, qubits: &[usize]) {
    assert!(!qubits.is_empty());

    let mask = qubit_mask(qubits, num_qubits);
    let phase = Complex64::new(-1.0, 0.0);

    apply_phase_on_basis_match(state, num_qubits, mask, mask, phase);
}

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

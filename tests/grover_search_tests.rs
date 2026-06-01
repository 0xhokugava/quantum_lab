use ndarray::ArrayD;
use num_complex::Complex64;
use quantum_lab::algorithms::grover_search::run_grover;
#[test]
fn run_grover_2_qubits() {
    let grover = run_grover(2, 2);
    let target_index = grover.target_index;
    let final_state = grover.final_state;
    assert_eq!(target_index, 2);
    assert!((grover.target_probability - 1.0).abs() < 1e-9);
    assert!((final_state[target_index].norm_sqr() - 1.0).abs() < 1e-9);

    final_state.iter().enumerate().for_each(|(i, ampl)| {
        if i != target_index {
            assert!(ampl.norm_sqr().abs() < 1e-9);
        }
    });

    assert!((total_probability(&final_state) - 1.0).abs() < 1e-9);
}
#[test]
fn run_grover_3_qubits() {
    let grover = run_grover(3, 5);
    let target_index = grover.target_index;
    let final_state = grover.final_state;
    assert_eq!(target_index, 5);
    assert_eq!(grover.grover_steps, 2);

    let initial_probability = 1.0 / ((1usize << 3) as f64);
    let target_probability = final_state[target_index].norm_sqr();

    assert!((target_probability - grover.target_probability).abs() < 1e-9);
    assert!(target_probability > initial_probability);
    assert!(target_probability > 0.9);

    final_state.iter().enumerate().for_each(|(i, ampl)| {
        if i != target_index {
            assert!(ampl.norm_sqr() < target_probability);
        }
    });

    assert!((total_probability(&final_state) - 1.0).abs() < 1e-9);
}

fn total_probability(final_state: &ArrayD<Complex64>) -> f64 {
    final_state.iter().map(|ampl| ampl.norm_sqr()).sum()
}

use eigenon::algorithms::grover_search::run_grover;
use ndarray::ArrayD;
use num_complex::Complex64;
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

#[test]
fn run_grover_4_qubits() {
    let grover = run_grover(4, 10);
    let target_index = grover.target_index;
    let final_state = grover.final_state;
    assert_eq!(target_index, 10);
    assert_eq!(grover.grover_steps, 3);

    let initial_probability = 1.0 / ((1usize << 4) as f64);
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

#[test]
fn run_grover_uses_recommended_steps() {
    let grover = run_grover(2, 2);
    assert_eq!(grover.grover_steps, 1);
    let grover = run_grover(3, 5);
    assert_eq!(grover.grover_steps, 2);
    let grover = run_grover(4, 10);
    assert_eq!(grover.grover_steps, 3);
    let grover = run_grover(5, 17);
    assert_eq!(grover.grover_steps, 4);
}

#[test]
#[should_panic]
fn invalid_target_index() {
    run_grover(3, 8);
}

#[test]
#[should_panic]
fn zero_qubits() {
    run_grover(0, 0);
}

fn total_probability(final_state: &ArrayD<Complex64>) -> f64 {
    final_state.iter().map(|ampl| ampl.norm_sqr()).sum()
}

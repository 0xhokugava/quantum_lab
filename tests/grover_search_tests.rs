use quantum_lab::algorithms::grover_search::run_grover;
#[test]
fn run_grover_2_qubit() {
    let grover = run_grover(2, 2);
    let target_index = grover.target_index;
    assert_eq!(target_index, 2);
    assert!((grover.target_probability - 1.0).abs() < 1e-9);
    assert!((grover.final_state[2].norm_sqr() - 1.0).abs() < 1e-9);

    grover.final_state.iter().enumerate().for_each(|(i, ampl)| {
        if i != target_index {
            assert!(ampl.norm_sqr().abs() < 1e-9);
        }
    });
}

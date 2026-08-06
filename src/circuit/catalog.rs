use crate::algorithms::deutsch::DeutschOracle;
use crate::algorithms::deutsch_jozsa::DeutschJozsaOracle;
use crate::algorithms::grover_search::recommended_grover_steps;
use crate::circuit::core::Circuit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BellState {
    PhiPlus,
    PhiMinus,
    PsiPlus,
    PsiMinus,
}

pub(crate) const QUERY: usize = 1;
pub(crate) const ANSWER: usize = 0;

pub fn bell(state: BellState) -> Circuit {
    let mut circuit = Circuit::new(2);

    circuit.h(0);
    circuit.cnot(0, 1);

    if matches!(state, BellState::PsiPlus | BellState::PsiMinus) {
        circuit.x(1);
    }

    if matches!(state, BellState::PhiMinus | BellState::PsiMinus) {
        circuit.z(0);
    }

    circuit
}

pub fn deutsch_circuit(oracle: DeutschOracle) -> Circuit {
    let mut circuit = Circuit::new(2);

    circuit.x(ANSWER);
    circuit.h(QUERY);
    circuit.h(ANSWER);

    oracle.apply(&mut circuit);

    circuit.h(QUERY);

    circuit
}

pub fn deutsch_jozsa_circuit(num_query_qubits: usize, oracle: DeutschJozsaOracle) -> Circuit {
    assert!(
        num_query_qubits > 0,
        "Deutsch-Jozsa requires at least one query qubit"
    );
    let total_qubits = num_query_qubits + 1;
    let mut circuit = Circuit::new(total_qubits);
    circuit.x(ANSWER);

    crate::algorithms::deutsch_jozsa::apply_h_to_queries(&mut circuit, num_query_qubits);

    circuit.h(ANSWER);

    oracle.apply(&mut circuit, num_query_qubits);

    crate::algorithms::deutsch_jozsa::apply_h_to_queries(&mut circuit, num_query_qubits);

    circuit
}

pub fn grover_circuit(num_qubits: usize, target_index: usize) -> Circuit {
    assert!(num_qubits > 0);
    assert!(target_index < (1usize << num_qubits));

    let grover_steps = recommended_grover_steps(num_qubits);
    let mut circuit = Circuit::new(num_qubits);

    circuit.h_all();

    for _ in 0..grover_steps {
        circuit.phase_oracle(target_index);
        circuit.diffusion();
    }

    circuit
}

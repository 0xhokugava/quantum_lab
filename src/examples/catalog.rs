use crate::algorithms::deutsch::DeutschOracle;
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

use crate::circuit::core::Circuit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BellState {
    PhiPlus,
    PhiMinus,
    PsiPlus,
    PsiMinus,
}

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

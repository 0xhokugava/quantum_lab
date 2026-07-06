use crate::circuit::core::Circuit;
use crate::circuit::operation::{GateKind, Operation};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenQasmExportError {
    UnsupportedOperation(Operation),
}

impl fmt::Display for OpenQasmExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenQasmExportError::UnsupportedOperation(operation) => {
                write!(f, "Unsupported OpenQASM operation: {:?}", operation)
            }
        }
    }
}

impl std::error::Error for OpenQasmExportError {}

pub fn export_openqasm2(circuit: &Circuit) -> Result<String, OpenQasmExportError> {
    let mut output = String::new();

    output.push_str("OPENQASM 2.0;\n");
    output.push_str("include \"qelib1.inc\";\n\n");
    output.push_str(&format!("qreg q[{}];\n\n", circuit.n_qubits()));

    for operation in circuit.operations() {
        match operation {
            Operation::SingleQubit { gate, target } => {
                output.push_str(&format!("{} q[{}];\n", openqasm_gate_name(*gate), target));
            }

            Operation::Cnot { control, target } => {
                output.push_str(&format!("cx q[{}], q[{}];\n", control, target));
            }

            Operation::Cz { control, target } => {
                output.push_str(&format!("cz q[{}], q[{}];\n", control, target));
            }

            Operation::Mcx { .. } | Operation::Mcz { .. } => {
                return Err(OpenQasmExportError::UnsupportedOperation(operation.clone()));
            }
        }
    }

    Ok(output)
}

fn openqasm_gate_name(gate: GateKind) -> &'static str {
    match gate {
        GateKind::X => "x",
        GateKind::Y => "y",
        GateKind::Z => "z",
        GateKind::H => "h",
        GateKind::S => "s",
        GateKind::T => "t",
    }
}

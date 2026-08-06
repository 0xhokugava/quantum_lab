use quantum_lab::circuit::catalog;
use quantum_lab::circuit::catalog::BellState;
use quantum_lab::circuit::core::Circuit;
use quantum_lab::circuit::operation::Operation;
use quantum_lab::export::openqasm::{OpenQasmExportError, export_openqasm2};

#[test]
fn exports_empty_circuit() {
    let circuit = Circuit::new(2);

    let qasm = export_openqasm2(&circuit).unwrap();

    assert_eq!(
        qasm,
        "OPENQASM 2.0;\n\
             include \"qelib1.inc\";\n\n\
             qreg q[2];\n\n"
    );
}

#[test]
fn exports_single_qubit_gates() {
    let mut circuit = Circuit::new(1);
    circuit.h(0).x(0).y(0).z(0).s(0).t(0);

    let qasm = export_openqasm2(&circuit).unwrap();

    assert_eq!(
        qasm,
        "OPENQASM 2.0;\n\
             include \"qelib1.inc\";\n\n\
             qreg q[1];\n\n\
             h q[0];\n\
             x q[0];\n\
             y q[0];\n\
             z q[0];\n\
             s q[0];\n\
             t q[0];\n"
    );
}

#[test]
fn exports_bell_circuit() {
    let circuit = catalog::bell(BellState::PhiPlus);
    let qasm = export_openqasm2(&circuit).unwrap();

    assert_eq!(
        qasm,
        "OPENQASM 2.0;\n\
             include \"qelib1.inc\";\n\n\
             qreg q[2];\n\n\
             h q[0];\n\
             cx q[0], q[1];\n"
    );
}

#[test]
fn exports_cz() {
    let mut circuit = Circuit::new(2);
    circuit.h(0).cz(0, 1);

    let qasm = export_openqasm2(&circuit).unwrap();

    assert_eq!(
        qasm,
        "OPENQASM 2.0;\n\
             include \"qelib1.inc\";\n\n\
             qreg q[2];\n\n\
             h q[0];\n\
             cz q[0], q[1];\n"
    );
}

#[test]
fn rejects_mcx_for_now() {
    let mut circuit = Circuit::new(3);
    circuit.mcx(&[0, 1], 2);

    let result = export_openqasm2(&circuit);

    assert!(matches!(
        result,
        Err(OpenQasmExportError::UnsupportedOperation(
            Operation::Mcx { .. }
        ))
    ));
}

#[test]
fn rejects_mcz_for_now() {
    let mut circuit = Circuit::new(3);
    circuit.mcz(&[0, 1], 2);

    let result = export_openqasm2(&circuit);

    assert!(matches!(
        result,
        Err(OpenQasmExportError::UnsupportedOperation(
            Operation::Mcz { .. }
        ))
    ));
}

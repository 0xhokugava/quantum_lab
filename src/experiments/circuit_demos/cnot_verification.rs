use crate::engine::utils::{run_circuit, to_dirac};

/// Runs a CNOT gate logic demonstration using the high-level `Circuit` API.
///
/// This experiment checks the basic controlled-NOT behavior:
/// - `CNOT|10>` flips the target qubit because the control qubit is `1`
/// - `CNOT|01>` leaves the state unchanged because the control qubit is `0`
///
/// Argument order follows the simulator convention:
/// `cnot(control, target)`.
pub fn run() {
    println!("\nCNOT Gate Logic:\n");
    let cnot_res_10 = run_circuit(2, |c| {
        c.x(1).cnot(1, 0);
    });
    println!("   CNOT|10> = {} (Flip expected)", to_dirac(&cnot_res_10));
    let cnot_res_01 = run_circuit(2, |c| {
        c.x(0).cnot(1, 0);
    });
    println!(
        "   CNOT|01> = {} (No flip expected)",
        to_dirac(&cnot_res_01)
    );
}

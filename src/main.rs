mod cli;
mod gate_spec;
use clap::Parser;
use cli::{Cli, Commands, DemoCommand, VerifyCommand};
use quantum_lab::circuit::Circuit;
use quantum_lab::utils::to_dirac;
use quantum_lab::{algorithms, experiments};

fn main() -> Result<(), String> {
    println!("\n🐈  Quantum Lab 🐈‍⬛\n");

    let cli = Cli::parse();
    match cli.command {
        Commands::Run { qubits, gates } => {
            if qubits == 0 {
                return Err("Circuit must contain at least one qubit".to_string());
            }
            for gate in &gates {
                gate.validate(qubits)?;
            }
            let mut circuit = Circuit::new(qubits);
            for gate in &gates {
                gate.apply(&mut circuit);
            }
            let state = circuit.run();
            println!("Qubits: {}, Gates: {:?}", qubits, gates);
            println!("State: {}", to_dirac(&state));
        }
        Commands::Demo { command } => match command {
            DemoCommand::All => experiments::run_all_demos(),
            DemoCommand::SingleQubit => experiments::circuit_demos::single_qubit::run(),
            DemoCommand::TensorProduct => experiments::foundations::tensor_product_example::run(),
            DemoCommand::Cnot => experiments::circuit_demos::cnot_verification::run(),
            DemoCommand::Entanglement => experiments::circuit_demos::entanglement::run(),
            DemoCommand::Deutsch => algorithms::deutsch::run_deutsch_demo(),
            DemoCommand::DeutschJozsa => algorithms::deutsch_jozsa::run_deutsch_jozsa_demo(),
            DemoCommand::Grover => algorithms::grover_search::run_grover_demo(),
        },
        Commands::Verify { command } => match command {
            VerifyCommand::All => experiments::run_all_verifications(),
            VerifyCommand::Inplace => experiments::engine_verification::inplace_verification::run(),
            VerifyCommand::CnotInplace => {
                experiments::engine_verification::cnot_inplace_verification::run()
            }
        },
    }
    Ok(())
}

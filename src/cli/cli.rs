use super::gate_spec::GateSpec;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum DemoCommand {
    All,
    SingleQubit,
    TensorProduct,
    Cnot,
    Entanglement,
    Deutsch,
    DeutschJozsa,
    Grover,
}

#[derive(Subcommand)]
pub enum VerifyCommand {
    All,
    Inplace,
    CnotInplace,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run an existing demo
    Demo {
        #[command(subcommand)]
        command: DemoCommand,
    },
    /// Run verifications
    Verify {
        #[command(subcommand)]
        command: VerifyCommand,
    },
    /// Build and execute a quantum circuit
    Run {
        #[arg(short, long)]
        qubits: usize,
        #[arg(long = "gate", value_name = "GATE")]
        gates: Vec<GateSpec>,
    },
    /// Export a quantum circuit to OpenQASM 2.0
    #[command(name = "export-openqasm")]
    ExportOpenqasm {
        #[arg(short, long)]
        qubits: usize,

        #[arg(long = "gate", value_name = "GATE")]
        gates: Vec<GateSpec>,
    },
}

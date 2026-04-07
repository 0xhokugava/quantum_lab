# Quantum Lab

A modular quantum computing simulator built with Rust.

## Goals
The project aims to simulate quantum circuits from scratch to understand the underlying linear algebra and quantum logic.

## Features
- **State Representation**: Dirac (Bra-ket) notation output for any number of qubits.
- **Math Engine**: Manual implementation of the Kronecker (tensor) product to expand state space.
- **Gate Library**: Single-qubit (X, H) and multi-qubit (CNOT) gates.
- **Statistical Engine**: High-performance measurement simulation (shots) to verify superposition.

## Quick Start
```bash
cargo run
```

## 📁 Project Structure
- `src/constants.rs`: Quantum gates and basis states definitions.
- `src/ops.rs`: Mathematical operations.
- `src/formatting.rs`: Visualization tools for quantum states.
- `src/measurement.rs`: Simulation of the wave function collapse.


## Current Progress
- Single qubit gates (X, H)
- Multi-qubit state vectors (Tensor Product)
- Controlled-NOT (CNOT) logic

## Next Steps
- Bell State (Entanglement)

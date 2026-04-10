# Quantum Lab

A modular quantum computing simulator built with Rust.

## Goals
The project aims to simulate quantum circuits from scratch to understand the underlying linear algebra and quantum logic.

## Features
- **State Representation**: Dirac (Bra-ket) notation output for any number of qubits.
- **Math Engine**: Generic implementation of the Kronecker (tensor) product supporting vectors and matrices.
- **Gate Library**: Single-qubit (X, H, I) and multi-qubit (CNOT) gates.
- **Statistical Engine**: High-performance measurement simulation (shots) to verify superposition.

## Quick Start
To run the latest research experiments:
```bash
cargo run
```
To run the verification test suite:
```bash
cargo test
```

## 📁 Project Structure
- `src/main.rs`: Entry point for the quantum simulator.
- `src/lib.rs`: Quantum simulator library.
- `src/constants.rs`: Quantum gates and basis states definitions.
- `src/ops.rs`: Mathematical operations.
- `src/formatting.rs`: Visualization tools for quantum states.
- `src/measurement.rs`: Simulation of the wave function collapse.
- `src/experiments`: Modular research sessions (Single qubit, Entanglement).
- `tests/`: Integration tests.


## Current Progress
- [x] Universal Tensor Product (Generics)
- [x] Single qubit gates (X, H, Identity)
- [x] Multi-qubit state vectors
- [x] Controlled-NOT (CNOT) logic
- [x] Quantum Entanglement (Bell State)

## Next Steps
- Multi-qubit measurement decoding: Transition from index-based measurement to bit-string outcomes (e.g., `0` -> `"00"`, `3` -> `"11"`).
- Complex number support: Refactor the state vector and gates to use `num-complex` for full quantum phase simulation.
- Quantum Circuit API: Implement a higher-level abstraction to build circuits without manual matrix multiplication.
- Standard Algorithms: Implement Deutsch-Jozsa and Grover's search as verification experiments.
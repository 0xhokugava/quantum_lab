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
To run performance benchmarks:
```bash
cargo bench
```

## 📁 Project Structure
- `benches/`: Performance benchmarks.
- `src/experiments`: Modular research sessions (Single qubit, Entanglement etc.).
- `src/lib.rs`: Quantum simulator library.
- `src/main.rs`: Entry point for the quantum simulator.
- `src/constants.rs`: Quantum gates and basis states definitions.
- `src/measurement.rs`: Simulation of the wave function collapse.
- `src/ops.rs`: Mathematical operations.
- `src/utils.rs`: Visualization tools for quantum states.
- `tests/`: Integration tests.

## Performance & Limitations
- Benchmarks were conducted up to 12 qubits. However, given the O(2^(2n)) scaling, the dense matrix-vector approach is expected to become impractical beyond ~16 qubits due to computational and memory constraints.

## Current Progress
- [x] Universal Tensor Product (Generics)
- [x] Single qubit gates (X, H, Identity)
- [x] Multi-qubit state vectors
- [x] Controlled-NOT (CNOT) logic
- [x] Quantum Entanglement (Bell State)
- [x] Multi-qubit measurement decoding and statistical analysis
- [x] Complex number support
- [x] Implemented Pauli and phase gates
- [x] In-place Gate Application (Matrix-free logic)
- [x] Performance benchmarking for time scaling and speedup vs matrix-based approach

## Next Steps
- Implement optimized controlled gates (CNOT, CZ) without constructing global matrices.
- Quantum Circuit API: Implement a higher-level abstraction to build circuits without manual matrix multiplication.
- Standard Algorithms: Implement Deutsch-Jozsa and Grover's search as verification experiments.
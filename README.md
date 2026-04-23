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
- `src/main.rs`: Entry point for the quantum simulator.
- `src/lib.rs`: Quantum simulator library.
- `src/constants.rs`: Quantum gates and basis states definitions.
- `src/ops.rs`: Mathematical operations.
- `src/formatting.rs`: Visualization tools for quantum states.
- `src/measurement.rs`: Simulation of the wave function collapse.
- `src/experiments`: Modular research sessions (Single qubit, Entanglement).
- `tests/`: Integration tests.

## Performance & Limitations
- The benchmark results show exponential growth in execution time as the number of qubits increases, with ~16× slowdown for every additional 2 qubits. Performance remains stable across runs, confirming correctness. However, the dense matrix-vector approach does not scale and becomes impractical beyond ~16 qubits due to computational and memory constraints.


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

## Next Steps
- Performance benchmarking using criterion to analyze scaling and memory efficiency vs matrix-based methods.
- Implement optimized controlled gates (CNOT, CZ) without constructing global matrices.
- Quantum Circuit API: Implement a higher-level abstraction to build circuits without manual matrix multiplication.
- Standard Algorithms: Implement Deutsch-Jozsa and Grover's search as verification experiments.
# Quantum Lab

A modular quantum state-vector simulator built from scratch in Rust.

## Goals

The goal of this project is to build a quantum state-vector simulator from scratch in Rust.

The focus is not only on reproducing quantum circuit behavior, but also on understanding the underlying linear algebra, state-vector representation, and performance trade-offs between dense matrix-based execution and matrix-free in-place updates.

The long-term direction is to grow the simulator from a low-level educational implementation into a small usable framework with a Circuit API, CLI examples, benchmarks, and standard algorithm demos.

## Features
- **State Representation**: Dirac (Bra-ket) notation output for multi-qubit state vectors.
- **Math Engine**: Generic implementation of the Kronecker (tensor) product for vectors and matrices.
- **Gate Library**: Single-qubit gates (X, Y, Z, H, S, T, I) and two-qubit CNOT gate.
- **Measurement Engine**: Shot-based measurement simulation for verifying probability distributions.
- **In-place Execution**: Matrix-free gate application directly on the state vector.
- **Generic k-qubit Gate Application**: Support for arbitrary local `2^k × 2^k` gates applied in-place.
- **Circuit API**: High-level abstraction for building and executing quantum circuits declaratively.
- **Benchmarking**: Criterion-based benchmarks comparing dense matrix-based execution with in-place execution.

## Architecture Status

The simulator now has three main abstraction layers:

- **Core Engine**: low-level state-vector operations, tensor products, measurement, and in-place gate application.
- **Gate Execution Layer**: generic `k`-qubit in-place gate application using local blocks of amplitudes.
- **Circuit API**: high-level interface for building and running circuits declaratively.

Example:

``` rust
let mut circuit = Circuit::new(2);

circuit.h(1).cnot(0, 1);

let state = circuit.run();
```

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
- `src/circuit.rs`: High-level quantum circuit abstraction.
- `src/constants.rs`: Quantum gates and basis state definitions.
- `src/experiments/`: Modular educational and verification experiments.
    - `circuit_demos/`: User-facing demos built with the Circuit API.
    - `engine_verification/`: Low-level verification experiments for in-place execution.
    - `foundations/`: Educational examples for tensor products and basic concepts.
- `src/lib.rs`: Quantum simulator library.
- `src/main.rs`: Entry point for running experiments.
- `src/measurement.rs`: Shot-based measurement simulation.
- `src/ops.rs`: Core mathematical and state-vector operations.
- `src/utils.rs`: Formatting, helpers, and state comparison utilities.
- `tests/`: Integration tests.

## Performance & Limitations

The simulator started with a dense matrix-based approach, where applying a gate required constructing a full `2^n × 2^n` operator. This is useful as a correctness baseline but scales poorly in both time and memory.

The current implementation supports matrix-free in-place gate application directly on the state vector. This reduces gate application from global matrix construction to local state-vector updates.

Benchmarks currently compare:
- dense matrix-based gate application
- optimized in-place single-qubit gate application
- optimized in-place CNOT gate application

Benchmarks have been run up to 12 qubits to demonstrate the scaling difference without turning the benchmark suite into a CPU stress test.

## Current Progress
- [x] Universal tensor product implementation
- [x] Multi-qubit state-vector representation
- [x] Dirac notation output
- [x] Single-qubit gates: X, Y, Z, H, S, T, Identity
- [x] CNOT gate implementation
- [x] Bell state / entanglement experiment
- [x] Multi-qubit measurement decoding and statistical analysis
- [x] Complex number support
- [x] Matrix-free in-place single-qubit gate application
- [x] Matrix-free in-place CNOT gate application
- [x] Generic in-place `k`-qubit gate application
- [x] Dense full-operator baseline for correctness tests
- [x] Performance benchmarks for matrix-based vs. in-place execution
- [x] Initial high-level Circuit API
- [x] Circuit API wrappers for basic gates
- [x] Circuit-based Bell state and single-qubit experiments
- [x] Experiment structure reorganized by abstraction level
- [x] Deutsch algorithm demo using the Circuit API (`n = 1` Deutsch–Jozsa case)

## Next Steps
- Finalize the basic Circuit API and keep measurement as a separate layer for now.
- Add a small CLI layer for running demos and experiments from the command line.
- Migrate educational experiments into CLI/examples.
- Implement the Deutsch–Jozsa algorithm as the first algorithm-level demo.
- Implement Grover's search as a more advanced algorithm-level demo.
- Add memory usage benchmarks vs. qubit count.
- Improve documentation and comments across the project.
- Explore parallelization only after the core API matures.

## Development Roadmap

Short-term focus:

1. Stabilize the basic Circuit API.
2. Add CLI support for running examples.
3. Implement Deutsch – Jozsa as the first full algorithm demo.
4. Implement Grover search.
5. Add memory benchmarks.
6. Revisit parallel in-place execution after the core architecture becomes stable.
# Roadmap

## Completed

* [x] Complex state-vector representation
* [x] Tensor products and dense matrix baseline
* [x] Matrix-free single-qubit gate execution
* [x] Matrix-free CNOT execution
* [x] Generic in-place `k`-qubit gate application
* [x] Circuit API
* [x] X, Y, Z, H, S, T, I, CNOT and CZ gates
* [x] Dirac notation output
* [x] Shot-based measurement
* [x] Deutsch algorithm
* [x] Deutsch–Jozsa algorithm
* [x] Grover search
* [x] Qiskit validation for Deutsch–Jozsa
* [x] Criterion benchmarks
* [x] `qlab` command-line interface
* [x] Custom circuit execution through CLI

## Current Focus

### Controlled Gate Primitives

* [ ] generic controlled single-qubit gate
* [ ] multi-controlled X
* [ ] multi-controlled Z
* [ ] validation for duplicate controls and invalid targets
* [ ] dense-vs-matrix-free correctness tests

### Gate-Level Grover

* [ ] build the oracle from primitive gates
* [ ] build diffusion from primitive gates
* [ ] preserve current Grover results
* [ ] validate the final circuit against Qiskit

## Next

* [ ] introduce an internal circuit representation
* [ ] improve validation and circuit reports
* [ ] add basic OpenQASM export
* [ ] add basic OpenQASM import
* [ ] document qubit-ordering conventions more clearly

## Later

* [ ] Qiskit adapter
* [ ] Braket or PennyLane adapter
* [ ] parameterized rotation gates
* [ ] circuit-gradient experiments
* [ ] memory benchmarks by qubit count
* [ ] parallel matrix-free execution experiments

## Current Development Order

1. Controlled and multi-controlled primitives
2. Gate-level Grover decomposition
3. Grover validation against Qiskit
4. Internal circuit representation
5. Validation reports
6. OpenQASM support
7. Initial SDK adapter
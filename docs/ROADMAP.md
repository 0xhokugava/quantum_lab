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
* [x] `eigenon` command-line interface
* [x] Custom circuit execution through CLI
* [x] MCX and MCZ gates through Circuit API
* [x] MCX and MCZ gate syntax through CLI
* [x] Gate-level Grover oracle built from primitive gates
* [x] Gate-level Grover diffusion built from primitive gates
* [x] Preserved existing Grover result after gate-level decomposition
* [x] Qiskit validation for gate-level Grover

## Current Focus

### Source Module Layout

* [ ] refactor source files into clearer modules
* [ ] preserve current behavior
* [ ] keep all tests passing

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
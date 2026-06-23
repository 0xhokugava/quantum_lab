# Contributing

Quantum Lab is a small Rust-based quantum circuit simulation and validation toolkit.

Contributions are welcome, especially in the areas of:

* tests and edge cases
* documentation
* CLI examples
* circuit validation
* OpenQASM export/import
* small refactors that preserve behavior

## Development setup

```bash
git clone https://github.com/0xhokugava/quantum_lab.git
cd quantum_lab
cargo test
cargo fmt
```

## Before opening a pull request

Please make sure:

* `cargo fmt` passes
* `cargo test` passes
* the change is focused and does not mix unrelated refactors
* behavior changes are covered by tests when possible

## Project conventions

* qubit `0` is the least significant bit
* printed basis states use standard binary order
* controlled gates use `control, target` order
* validation scripts live outside the simulator runtime

## Good first contributions

Good starting points are usually:

* documentation improvements
* additional tests
* CLI examples
* validation cases
* small cleanup tasks

For larger changes, please open an issue first.

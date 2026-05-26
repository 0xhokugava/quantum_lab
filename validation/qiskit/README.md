# Qiskit validation

This directory contains external Qiskit validation scripts for the Rust quantum simulator.

The code here is not part of the simulator runtime. It is used only as a reference check for algorithm-level behavior.

## Deutsch-Jozsa

`deutsch_jozsa.py` validates the same oracle cases as the Rust implementation:

- constant zero
- constant one
- balanced selected bit
- balanced parity

Expected behavior:

- constant oracle -> query register returns `|00...0>`
- balanced oracle -> query register does not return `|00...0>`

## Bit ordering

The Rust simulator uses little-endian qubit indexing: qubit `0` maps to the least significant bit.

Dirac output is printed in standard binary order, so qubit `0` appears as the rightmost bit.

## Run

```bash
python3 validation/qiskit/deutsch_jozsa.py
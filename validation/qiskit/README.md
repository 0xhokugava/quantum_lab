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

## Grover search

`grover.py` validates the gate-level Grover implementation against a Qiskit reference circuit.

Validated scenario:

* qubits: `3`
* target state: `|101>`
* target index: `5`
* Grover steps: `2`
* expected target probability: `0.9453125`

Quantum Lab result:

```text
Target state: |101> index=5
Target probability: 0.945313
Grover steps: 2
```

Qiskit result:

```text
Qiskit Grover validation

Target state: |101> index=5
Target amplitude: 0.972272 + 0.000000i
Target probability: 0.945312
Grover steps: 2

Validation passed.
```

The small display difference between `0.945313` and `0.945312` is only formatting precision. The exact expected value is `0.9453125`.

## Bit ordering

The Rust simulator uses little-endian qubit indexing: qubit `0` maps to the least significant bit.

Dirac output is printed in standard binary order, so qubit `0` appears as the rightmost bit.

## Run

```bash
python3 validation/qiskit/deutsch_jozsa.py
python3 validation/qiskit/grover.py
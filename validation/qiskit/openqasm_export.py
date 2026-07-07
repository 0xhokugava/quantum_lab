import sys

from qiskit import qasm2
from qiskit.quantum_info import Statevector


EPS = 1e-9
DISPLAY_EPS = 1e-12


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit(
            "Usage: python3 validation/qiskit/openqasm_export.py <path-to-qasm>"
        )

    qasm_path = sys.argv[1]

    qc = qasm2.load(qasm_path)
    state = Statevector.from_instruction(qc)
    probabilities = state.probabilities_dict()

    total_probability = sum(probabilities.values())
    assert abs(total_probability - 1.0) < EPS, (
        f"Expected probabilities to sum to 1.0, got {total_probability}"
    )

    print("OpenQASM export validation")
    print()
    print(f"QASM file: {qasm_path}")
    print(f"Qubits: {qc.num_qubits}")
    print()
    print(qc)
    print()
    print(state)
    print()
    print("Non-zero probabilities:")

    for basis, probability in sorted(probabilities.items()):
        if probability > DISPLAY_EPS:
            print(f"  P({basis}) = {probability}")

    print()
    print("Validation passed.")


if __name__ == "__main__":
    main()
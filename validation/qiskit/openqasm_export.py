import sys

from qiskit import qasm2
from qiskit.quantum_info import Statevector


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit("Usage: python3 validation/qiskit/openqasm_export.py <path-to-qasm>")

    qasm_path = sys.argv[1]

    qc = qasm2.load(qasm_path)
    state = Statevector.from_instruction(qc)
    probabilities = state.probabilities_dict()

    expected = {
        "00": 0.5,
        "11": 0.5,
    }

    for basis, probability in expected.items():
        actual = probabilities.get(basis, 0.0)
        assert abs(actual - probability) < 1e-9, (
            f"Expected P({basis})={probability}, got {actual}"
        )

    for basis, actual in probabilities.items():
        if basis not in expected:
            assert abs(actual) < 1e-9, f"Expected P({basis})=0, got {actual}"

    print("OpenQASM export validation")
    print()
    print(qc)
    print()
    print(state)
    print()
    print("Validation passed.")


if __name__ == "__main__":
    main()
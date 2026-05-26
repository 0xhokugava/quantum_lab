from qiskit import QuantumCircuit
from qiskit.quantum_info import Statevector


def constant_zero(n: int) -> QuantumCircuit:
    return QuantumCircuit(n + 1)


def constant_one(n: int) -> QuantumCircuit:
    qc = QuantumCircuit(n + 1)
    answer = n
    qc.x(answer)
    return qc


def balanced_selected_bit(n: int, query: int) -> QuantumCircuit:
    qc = QuantumCircuit(n + 1)
    answer = n
    qc.cx(query, answer)
    return qc


def balanced_parity(n: int) -> QuantumCircuit:
    qc = QuantumCircuit(n + 1)
    answer = n

    for query in range(n):
        qc.cx(query, answer)

    return qc


def deutsch_jozsa_circuit(n: int, oracle: QuantumCircuit) -> QuantumCircuit:
    answer = n
    qc = QuantumCircuit(n + 1)
    qc.x(answer)
    qc.h(range(n + 1))
    qc.compose(oracle, inplace=True)
    qc.h(range(n))

    return qc


def query_zero_probability(state: Statevector, n: int) -> float:
    probs = state.probabilities()
    query_mask = (1 << n) - 1

    p_zero = 0.0
    for basis_index, probability in enumerate(probs):
        query_bits = basis_index & query_mask
        if query_bits == 0:
            p_zero += probability

    return p_zero


def classify(p_zero: float) -> str:
    return "constant" if p_zero > 0.5 else "balanced"


def run_case(label: str, expected: str, n: int, oracle: QuantumCircuit) -> None:
    qc = deutsch_jozsa_circuit(n, oracle)
    state = Statevector.from_instruction(qc)

    p_zero = query_zero_probability(state, n)
    observed = classify(p_zero)

    assert observed == expected, (
        f"{label}: expected {expected}, got {observed}, "
        f"P(query = |00...0>) = {p_zero:.6f}"
    )

    print(f"{label}:")
    print(f"  expected: {expected}")
    print(f"  observed: {observed}")
    print(f"  P(query = |00...0>): {p_zero:.6f}")
    print()


def main() -> None:
    n = 3

    cases = [
        ("constant zero", "constant", constant_zero(n)),
        ("constant one", "constant", constant_one(n)),
        ("balanced selected bit", "balanced", balanced_selected_bit(n, query=0)),
        ("balanced parity", "balanced", balanced_parity(n)),
    ]

    for label, expected, oracle in cases:
        run_case(label, expected, n, oracle)


if __name__ == "__main__":
    main()

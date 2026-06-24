from math import floor, isclose, pi, sqrt
from qiskit import QuantumCircuit
from qiskit.quantum_info import Statevector


EXPECTED_TARGET_PROBABILITY = 0.9453125
ABS_TOLERANCE = 1e-9


def recommended_grover_steps(num_qubits: int) -> int:
    assert num_qubits > 0

    search_space_size = 1 << num_qubits
    return floor((pi / 4.0) * sqrt(search_space_size))


def target_label(target_index: int, num_qubits: int) -> str:
    return f"|{target_index:0{num_qubits}b}>"


def apply_h_all(circuit: QuantumCircuit, num_qubits: int) -> None:
    for qubit in range(num_qubits):
        circuit.h(qubit)


def apply_x_all(circuit: QuantumCircuit, num_qubits: int) -> None:
    for qubit in range(num_qubits):
        circuit.x(qubit)


def apply_phase_flip_all_ones(circuit: QuantumCircuit, num_qubits: int) -> None:
    if num_qubits == 1:
        circuit.z(0)
        return

    target = num_qubits - 1
    controls = list(range(target))

    circuit.h(target)
    circuit.mcx(controls, target)
    circuit.h(target)


def apply_phase_oracle(
        circuit: QuantumCircuit,
        num_qubits: int,
        target_index: int,
) -> None:
    assert target_index < (1 << num_qubits)

    for qubit in range(num_qubits):
        if ((target_index >> qubit) & 1) == 0:
            circuit.x(qubit)

    apply_phase_flip_all_ones(circuit, num_qubits)

    for qubit in range(num_qubits):
        if ((target_index >> qubit) & 1) == 0:
            circuit.x(qubit)


def apply_diffusion(circuit: QuantumCircuit, num_qubits: int) -> None:
    apply_h_all(circuit, num_qubits)
    apply_x_all(circuit, num_qubits)
    apply_phase_flip_all_ones(circuit, num_qubits)
    apply_x_all(circuit, num_qubits)
    apply_h_all(circuit, num_qubits)


def build_grover_circuit(
        num_qubits: int,
        target_index: int,
) -> tuple[QuantumCircuit, int]:
    assert num_qubits > 0
    assert target_index < (1 << num_qubits)

    grover_steps = recommended_grover_steps(num_qubits)

    circuit = QuantumCircuit(num_qubits)
    apply_h_all(circuit, num_qubits)

    for _ in range(grover_steps):
        apply_phase_oracle(circuit, num_qubits, target_index)
        apply_diffusion(circuit, num_qubits)

    return circuit, grover_steps


def main() -> None:
    num_qubits = 3
    target_index = 5

    circuit, grover_steps = build_grover_circuit(num_qubits, target_index)
    state = Statevector.from_instruction(circuit)

    target_amplitude = state.data[target_index]
    target_probability = abs(target_amplitude) ** 2

    print("Qiskit Grover validation")
    print()
    print(f"Target state: {target_label(target_index, num_qubits)} index={target_index}")
    print(f"Target amplitude: {target_amplitude.real:.6f} + {target_amplitude.imag:.6f}i")
    print(f"Target probability: {target_probability:.6f}")
    print(f"Grover steps: {grover_steps}")

    assert isclose(
        target_probability,
        EXPECTED_TARGET_PROBABILITY,
        rel_tol=0.0,
        abs_tol=ABS_TOLERANCE,
    ), (
        f"Expected target probability {EXPECTED_TARGET_PROBABILITY}, "
        f"got {target_probability}"
    )

    print()
    print("Validation passed.")


if __name__ == "__main__":
    main()
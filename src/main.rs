use quantum_lab::constants::{gate_cnot, gate_h, gate_x, q0, q1};
use quantum_lab::formatting::*;
use quantum_lab::measurement::*;
use quantum_lab::ops::*;

fn main() {
    println!("Hello, quantum world!");
    // |0> (q0) is the ground state. Applying X-gate (NOT) flips it to |1>.
    let new_state_x = gate_x().dot(&q0());
    println!("new_state_x Dirac notation: {}", to_dirac(&new_state_x));

    // H-gate (Hadamard) transforms basis states into balanced superposition.
    // It creates a "quantum coin flip" state.
    let new_state_h = gate_h().dot(&q0());
    println!("new_state_h Dirac notation: {}", to_dirac(&new_state_h));

    // Sequence: X then H. This results in the |- > state (+0.707|0> -0.707|1>).
    // It has the same 50/50 probabilities as H|0>, but a different "phase" (the minus sign).
    let final_state_x = gate_h().dot(&new_state_x);
    println!("final_state_x Dirac notation: {}", to_dirac(&final_state_x));

    // This line runs a million simulations of the qubit's measurement to confirm
    // that it was in a superposition state.
    // 50/50 result proves that our mathematical model correctly predicts the particle's behavior.
    let (p_one, p_zero) = test_measure(&final_state_x, 1_000_000);
    println!("{0:.2}% vs {1:.2}%", p_one, p_zero);

    // Combining two independent qubits into a single 4-dimensional system (2^2 states).
    // Here we represent the state |1> (first) and |0> (second) as |10>.
    let tensor_result = tensor_product(&q1(), &q0());
    println!(
        "Dirac notation of the tensor product: {}",
        to_dirac(&tensor_result)
    );

    // CNOT Logic Verification

    // Case 1: Control qubit is |1> (Excited state).
    // The CNOT gate should flip the target qubit from |0> to |1>.
    let state_10 = tensor_product(&q1(), &q0());
    let result = gate_cnot().dot(&state_10);
    // Expected output: |11>
    println!("Input |10> -> CNOT -> Output: {}", to_dirac(&result));

    // Case 2: Control qubit is |0> (Ground state).
    // The CNOT gate should do nothing, leaving the target qubit as |1>.
    let state_01 = tensor_product(&q0(), &q1());
    let result = gate_cnot().dot(&state_01);
    // Expected output: |01> (No change)
    println!("Input |01> -> CNOT -> Output: {}", to_dirac(&result));
}

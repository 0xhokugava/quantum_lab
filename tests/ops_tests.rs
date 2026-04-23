use ndarray::Array1;
use ndarray::linalg::Dot;
use num_complex::Complex64;
use quantum_lab::constants::{gate_h, gate_s, gate_t, gate_x, gate_y, gate_z, identity, q0, q1};
use quantum_lab::ops::{apply_gate_inplace, tensor_product};

fn to_c64(re: f64) -> Complex64 {
    Complex64::new(re, 0.0)
}

#[test]
fn test_tensor_product() {
    let c_array = |re_vals: Vec<f64>| -> Array1<Complex64> {
        Array1::from_vec(re_vals.into_iter().map(|re| to_c64(re)).collect())
    };
    // Test |0> ⊗ |0> = |00> (index 0 is 1.0)
    let res00 = tensor_product(&q0(), &q0())
        .into_dimensionality::<ndarray::Ix1>()
        .expect("Should be a 1D vector");

    assert_eq!(res00, c_array(vec![1.0, 0.0, 0.0, 0.0]));

    // Test |0> ⊗ |1> = |01> (index 1 is 1.0)
    let res01 = tensor_product(&q0(), &q1())
        .into_dimensionality::<ndarray::Ix1>()
        .expect("Should be a 1D vector");
    assert_eq!(res01, c_array(vec![0.0, 1.0, 0.0, 0.0]));

    // Test |1> ⊗ |0> = |10> (index 2 is 1.0)
    let res10 = tensor_product(&q1(), &q0())
        .into_dimensionality::<ndarray::Ix1>()
        .expect("Should be a 1D vector");
    assert_eq!(res10, c_array(vec![0.0, 0.0, 1.0, 0.0]));

    // Test |1> ⊗ |1> = |11> (index 3 is 1.0)
    let res11 = tensor_product(&q1(), &q1())
        .into_dimensionality::<ndarray::Ix1>()
        .expect("Should be a 1D vector");
    assert_eq!(res11, c_array(vec![0.0, 0.0, 0.0, 1.0]));
}

#[test]
fn test_tensor_product_order_matters() {
    let res_01 = tensor_product(&q0(), &q1());
    let res_10 = tensor_product(&q1(), &q0());
    // |01> should NOT be equal to |10>
    assert_ne!(res_01, res_10);
}

#[test]
fn test_tensor_product_empty() {
    let empty_arr = Array1::from(vec![]);
    let res = tensor_product(&empty_arr, &q0());
    // The result should be empty if one of the inputs is empty
    assert_eq!(res.len(), 0);
}

#[test]
fn test_tensor_product_three_qubits() {
    // (|1> ⊗ |0>) ⊗ |0> = |100>
    // |100> in 8-dimensional space is index 4
    let res_2q = tensor_product(&q1(), &q0());
    let res_3q = tensor_product(&res_2q, &q0())
        .into_dimensionality::<ndarray::Ix1>()
        .expect("Three-qubit state must be a 1D vector");
    let mut expected = Array1::<Complex64>::zeros(8);
    expected[4] = to_c64(1.0);

    assert_eq!(res_3q, expected);
    assert_eq!(res_3q.len(), 8);
}

#[test]
fn test_apply_gate_inplace() {
    let state = tensor_product(&q0(), &q0());
    let gates = vec![
        (gate_h(), "H"),
        (gate_x(), "X"),
        (gate_y(), "Y"),
        (gate_z(), "Z"),
        (gate_s(), "S"),
        (gate_t(), "T"),
    ];

    let identity = identity();
    let n_qubits = state.len().ilog2() as usize;

    for target in 0..n_qubits {
        for (gate, name) in &gates {
            let mut state_inplace = state.clone();

            let full = match target {
                0 => tensor_product(&identity, &gate),
                1 => tensor_product(&gate, &identity),
                _ => unreachable!(),
            };

            let expected = full.dot(&state);
            apply_gate_inplace(&mut state_inplace, gate, target);
            for (i, (a, b)) in state_inplace.iter().zip(expected.iter()).enumerate() {
                assert!(
                    (a - b).norm() < 1e-10,
                    "Gate {} failed on target {}, index {}",
                    name,
                    target,
                    i
                );
            }
        }
    }
}

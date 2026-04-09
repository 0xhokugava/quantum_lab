use ndarray::Array1;

/// Computes the Kronecker product of two state vectors.
/// The resulting vector has length = v1.len * v2.len.
pub fn tensor_product(v1: &Array1<f64>, v2: &Array1<f64>) -> Array1<f64> {
    let mut res = Array1::zeros(v1.len() * v2.len());
    for i in 0..v1.len() {
        for j in 0..v2.len() {
            res[i * v2.len() + j] = v1[i] * v2[j];
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{q0, q1};
    use ndarray::array;

    #[test]
    fn test_tensor_product() {
        // Test |0> ⊗ |0> = |00> (index 0 is 1.0)
        let res00 = tensor_product(&q0(), &q0());
        assert_eq!(res00, array![1.0, 0.0, 0.0, 0.0]);

        // Test |0> ⊗ |1> = |01> (index 1 is 1.0)
        let res01 = tensor_product(&q0(), &q1());
        assert_eq!(res01, array![0.0, 1.0, 0.0, 0.0]);

        // Test |1> ⊗ |0> = |10> (index 2 is 1.0)
        let res10 = tensor_product(&q1(), &q0());
        assert_eq!(res10, array![0.0, 0.0, 1.0, 0.0]);

        // Test |1> ⊗ |1> = |11> (index 3 is 1.0)
        let res11 = tensor_product(&q1(), &q1());
        assert_eq!(res11, array![0.0, 0.0, 0.0, 1.0]);
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
        let res_3q = tensor_product(&res_2q, &q0());
        let mut expected = Array1::zeros(8);
        expected[4] = 1.0;

        assert_eq!(res_3q, expected);
        assert_eq!(res_3q.len(), 8);
    }
}

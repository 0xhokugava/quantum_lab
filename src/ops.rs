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

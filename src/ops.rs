use ndarray::{Array, ArrayD, Dimension, IntoDimension, IxDyn};

/// Computes the universal Kronecker product (tensor product) of two arrays.
/// Works for state vectors (1D), gates/matrices (2D), and higher-dimensional arrays.
/// The resulting array's shape is the element-wise product of the input shapes.
pub fn tensor_product<D1, D2>(v1: &Array<f64, D1>, v2: &Array<f64, D2>) -> ArrayD<f64>
where
    D1: Dimension,
    D2: Dimension,
{
    let shape_v1 = v1.shape();
    let shape_v2 = v2.shape();

    let new_shape: Vec<usize> = shape_v1
        .iter()
        .zip(shape_v2.iter())
        .map(|(a, b)| a * b)
        .collect();

    let mut res = ArrayD::zeros(IxDyn(&new_shape));

    for (idx_v1, &val_v1) in v1.indexed_iter() {
        for (idx_v2, &val_v2) in v2.indexed_iter() {
            let dyn_idx_v1 = idx_v1.clone().into_dimension();
            let dyn_idx_v2 = idx_v2.clone().into_dimension();
            let mut new_idx = Vec::new();
            for i in 0..shape_v1.len() {
                new_idx.push(dyn_idx_v1[i] * shape_v2[i] + dyn_idx_v2[i]);
            }
            res[IxDyn(&new_idx)] = val_v1 * val_v2;
        }
    }

    res
}

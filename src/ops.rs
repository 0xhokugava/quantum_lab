use ndarray::{Array, Array2, ArrayD, Dimension, IntoDimension, IxDyn};
use num_complex::Complex64;

/// Computes the universal Kronecker product (tensor product) of two arrays.
/// Works for state vectors (1D), gates/matrices (2D), and higher-dimensional arrays.
/// The resulting array's shape is the element-wise product of the input shapes.
pub fn tensor_product<D1, D2>(
    v1: &Array<Complex64, D1>,
    v2: &Array<Complex64, D2>,
) -> ArrayD<Complex64>
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

/// Applies a single-qubit gate to a multi-qubit state vector in-place.
///
/// This is a high-performance, matrix-free implementation that manipulates
/// the state vector directly. It avoids the exponential memory overhead of
/// constructing a global operator matrix via Kronecker product.
///
/// The algorithm identifies and updates pairs of amplitudes (i0, i1)
/// that correspond to the target qubit's 0 and 1 states.
pub fn apply_gate_inplace(state: &mut ArrayD<Complex64>, gate: &Array2<Complex64>, target: usize) {
    let stride = 1 << target;
    let size = state.len();

    for i in (0..size).step_by(stride * 2) {
        for j in 0..stride {
            let i0 = i + j;
            let i1 = i0 + stride;

            let a = state[i0];
            let b = state[i1];

            state[i0] = gate[[0, 0]] * a + gate[[0, 1]] * b;
            state[i1] = gate[[1, 0]] * a + gate[[1, 1]] * b;
        }
    }
}

/// Applies a CNOT (controlled-NOT) gate to a multi-qubit state vector in-place.
///
/// This is a high-performance, matrix-free implementation that avoids
/// constructing the full 2^n × 2^n operator matrix. Instead, it directly
/// manipulates the state vector using bitwise operations.
///
/// The algorithm iterates over all basis state indices and identifies pairs
/// of amplitudes (i, j) that differ only in the target qubit. For indices where
/// the control qubit is set to 1, the corresponding amplitudes are swapped,
/// effectively performing a conditional bit-flip on the target qubit.
///
/// Each pair is processed exactly once using an ordering condition (i < j)
/// to avoid redundant swaps.
pub fn apply_cnot_inplace(state: &mut ArrayD<Complex64>, control: usize, target: usize) {
    assert_eq!(state.ndim(), 1);
    let target_mask = 1 << target;
    for i in 0..state.len() {
        if ((i >> control) & 1) == 1 {
            let j = i ^ target_mask;
            if i < j {
                state.swap(i, j);
            }
        }
    }
}

/// Applies an arbitrary k-qubit gate to a multi-qubit state vector in-place.
///
/// The gate must have the shape 2^k × 2^k, where k = targets.len().
/// The state is expected to be a flattened 1D state vector of length 2^n.
///
/// This is the generic version of the in-place gate application:
/// - k = 1 covers single-qubit gates
/// - k = 2 covers two-qubit gates such as CNOT
pub fn apply_k_qubit_gate_inplace(
    state: &mut ArrayD<Complex64>,
    gate: &Array2<Complex64>,
    targets: &[usize],
) {
    assert_eq!(state.ndim(), 1);

    let k = targets.len();
    let dim = 1 << k;

    assert_eq!(gate.shape(), &[dim, dim]);

    let size = state.len();

    let mut target_mask = 0usize;
    for &t in targets {
        target_mask |= 1 << t;
    }

    let mut indices = vec![0usize; dim];
    let mut block = vec![Complex64::new(0.0, 0.0); dim];
    let mut result = vec![Complex64::new(0.0, 0.0); dim];

    for base in 0..size {
        if base & target_mask != 0 {
            continue;
        }

        for j in 0..dim {
            let mut idx = base;

            for (bit_pos, &t) in targets.iter().enumerate() {
                if (j >> bit_pos) & 1 == 1 {
                    idx |= 1 << t;
                }
            }

            indices[j] = idx;
            block[j] = state[idx];
        }

        for r in 0..dim {
            let mut acc = Complex64::new(0.0, 0.0);

            for c in 0..dim {
                acc += gate[[r, c]] * block[c];
            }

            result[r] = acc;
        }

        for j in 0..dim {
            state[indices[j]] = result[j];
        }
    }
}

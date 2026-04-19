use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use ndarray::{Array1, Array2};
use num_complex::Complex64;
use rand::{RngExt, rng};

/// Benchmark for dense state-vector simulation.
///
/// Measures the execution time of applying a full 2^n × 2^n gate matrix
/// to a 2^n state vector using naive matrix-vector multiplication.
///
/// This benchmark runs across increasing qubit counts to observe
/// scalability and performance degradation (expected exponential growth).
///
/// Note:
/// - This represents the baseline (non-optimized) approach.
/// - Used to highlight the limitations of dense simulation
///   before introducing more efficient gate application methods.
fn random_state(n_qubits: usize) -> Array1<Complex64> {
    let size = 1 << n_qubits;
    let mut rng = rng();
    let mut v = Array1::from(
        (0..size)
            .map(|_| Complex64::new(rng.random::<f64>(), rng.random::<f64>()))
            .collect::<Vec<_>>(),
    );
    let norm: f64 = v.iter().map(|x| x.norm_sqr()).sum::<f64>().sqrt();
    v.mapv_inplace(|x| x / norm);
    v
}

fn apply_gate(g: &Array2<Complex64>, s: &Array1<Complex64>) -> Array1<Complex64> {
    g.dot(s)
}

fn bench_apply(c: &mut Criterion) {
    let mut group = c.benchmark_group("apply_gate_dense");

    for &n in &[4, 6, 8, 10, 12, 14] {
        let dim = 1 << n;

        let g = Array2::eye(dim).mapv(|x| Complex64::new(x, 0.0));
        let s = random_state(n);

        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &_n| {
            b.iter(|| {
                let _ = apply_gate(&g, &s);
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_apply);
criterion_main!(benches);

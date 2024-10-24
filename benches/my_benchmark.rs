use criterion::{criterion_group, criterion_main, Criterion};
use collatz_rust::collatz_length_brian;
use rayon::prelude::*;

pub fn fivemillserial() {
    let xs: Vec<i64> = (1..=5000000).collect();
    let ys: Vec<_>   = xs.iter().map(|&n| collatz_length_brian(n)).collect();
}


pub fn fivemillparallel() {
    let xs: Vec<i64> = (1..=5000000).collect();
    let ys: Vec<_>   = xs.par_iter().map(|&n| collatz_length_brian(n)).collect();
}

fn criterion_benchmark1(c: &mut Criterion) {
    c.bench_function("5 million serial", |b| b.iter(|| fivemillserial()));
}

fn criterion_benchmark2(c: &mut Criterion) {
    c.bench_function("5 million parallel", |b| b.iter(|| fivemillparallel()));
}
criterion_group!{
    name    = benches;
    config  = Criterion::default().sample_size(10);
    targets = criterion_benchmark1, criterion_benchmark2
}
criterion_main!(benches);

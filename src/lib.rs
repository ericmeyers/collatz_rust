
use itertools::iterate;
use itertools::Itertools;
use rayon::prelude::*;


pub fn collatz_next(n: &i64) -> i64 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

pub fn collatz_length(n: &i64) -> i64 {
    let v: Vec<_> = iterate(*n, collatz_next)
        .take_while_inclusive(|&n| n > 1)
        .collect();
    v.len() as i64
}

pub fn collatz_length_brian(n: i64) -> i64 {
    if n == 1 {
        1
    } else if n % 2 == 0 {
        1 + collatz_length_brian(n / 2)
    } else {
        1 + collatz_length_brian(3 * n + 1)
    }
}


pub fn fivemillserial() {
    let xs: Vec<i64> = (1..=50000).collect();
    let ys: Vec<_>   = xs.iter().map(|&n| collatz_length_brian(n)).collect();
}


pub fn fivemillparallel() {
    let xs: Vec<i64> = (1..=50000).collect();
    let ys: Vec<_>   = xs.par_iter().map(|&n| collatz_length_brian(n)).collect();
}


#[cfg(test)]
mod test {
  use criterion::{criterion_group, criterion_main, Criterion};




}

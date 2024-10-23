
use itertools::iterate;
use itertools::Itertools;

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

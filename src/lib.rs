
use itertools::iterate;
use itertools::Itertools;
use tailcall::tailcall;

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

pub fn collatz_length_brian(n: i64) -> i32 {
// compiler fail if not in tco
    #[tailcall]
    fn tco_helper(n: i64, sum: i64) -> i32 {
        if n <= 1 {
            // just in case some hack asks for collatz_length(0)
            1 + sum as i32
        } else if n % 2 == 0 {
            tco_helper(n / 2, sum + 1)
        } else {
            tco_helper(3 * n + 1, sum + 1)
        }

    }

    tco_helper(n, 0)

}

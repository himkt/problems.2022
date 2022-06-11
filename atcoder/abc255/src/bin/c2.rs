pub fn lower_bound(range: std::ops::Range<i128>, prop: &dyn Fn(i128) -> bool) -> i128 {
    if prop(range.start) {
        return range.start;
    }

    let mut ng = range.start;
    let mut ok = range.end;

    while ok - ng > 1 {
        let middle = ng + (ok - ng) / 2;
        match prop(middle) {
            true => ok = middle,
            false => ng = middle,
        }
    }

    ok
}


#[allow(clippy::needless_range_loop)]
#[allow(clippy::vec_init_then_push)]
#[allow(clippy::assign_op_pattern)]
fn main() {
    let mut scanner = Scanner::new();
    let x: i128 = scanner.cin();
    let mut a: i128 = scanner.cin();
    let mut d: i128 = scanner.cin();
    let n: i128 = scanner.cin();

    // old: a_i = a + (x - 1) * d
    // new: a_i = (a * (n - 1) * d) + (x - 1) * (-d)
    //
    // ```python
    // >>> a = 5
    // >>> d = -1
    // >>> n = 5
    // >>> [a + (x - 1) * d for x in range(1, n+1)]
    // [5, 4, 3, 2, 1]
    // >>> a = a + (n - 1) * d
    // >>> d *= -1
    // >>> [a + (x - 1) * d for x in range(1, n+1)]
    // [1, 2, 3, 4, 5]
    // ```
    //
    if d < 0 {
        a = a + (n - 1) * d;
        d *= -1;
    }

    let f = |x: i128| a + (d * (x - 1));
    let mut candidates = vec![];

    for k in 1..100 {
        let q = k;
        if 1 <= q && q <= n {
            candidates.push((f(k) - x).abs());
        }
    }

    for k in 0..100 {
        let q = n - k;
        if 1 <= q && q <= n {
            candidates.push((f(n - k) - x).abs());
        }
    }

    let k_min = lower_bound(
        1..(n + 1),
        &|q| f(q) >= x,
    );
    for diff in -100..=100 {
        let q = k_min + diff;
        if 1 <= q && q <= n {
            candidates.push((f(q) - x).abs());
        }
    }

    let ans = candidates.iter().min().unwrap();
    println!("{}", ans);
}


use std::collections::VecDeque;
use std::io::{self, Write};
use std::str::FromStr;

pub struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
impl Scanner {
    fn new() -> Self {
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
    }

    pub fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

pub fn flush() {
    std::io::stdout().flush().unwrap();
}

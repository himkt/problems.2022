fn calc_cost(m: i64, a: i64, b: i64, c: i64) -> i64 {
    let d1 = m.max(a) - m.min(a);
    let d2 = m.max(b) - m.min(b);
    let d3 = m.max(c) - m.min(c);
    d1 + d2 + d3
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let t: usize = scanner.cin();

    for _ in 0..t {
        let n: usize = scanner.cin();
        let mut sticks: Vec<i64> = scanner.vec(n);
        sticks.sort_unstable();

        let mut ans = 1_000_000_000_000_000;

        for i in 0..(n - 2) {
            let a = sticks[i];
            let b = sticks[i + 1];
            let c = sticks[i + 2];

            for mu in [a, b, c] {
                ans = ans.min(calc_cost(mu, a, b, c))
            }
        }
        println!("{}", ans);
    }
}

use std::{io::Write};
pub struct Scanner {
    buffer: std::collections::VecDeque<String>,
    buf: String,
}
#[allow(clippy::new_without_default)]
impl Scanner {
    pub fn new() -> Self {
        Scanner {
            buffer: std::collections::VecDeque::new(),
            buf: String::new(),
        }
    }
    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        if !self.buffer.is_empty() {
            return self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap();
        }
        self.buf.truncate(0);
        std::io::stdin().read_line(&mut self.buf).ok();
        self.buf
            .to_owned()
            .split_whitespace()
            .for_each(|x| self.buffer.push_back(String::from(x)));
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
    pub fn flush(&self) {
        std::io::stdout().flush().unwrap();
    }
}

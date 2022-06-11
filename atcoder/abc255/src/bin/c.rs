#[allow(clippy::needless_range_loop)]
#[allow(clippy::vec_init_then_push)]
fn main() {
    let mut scanner = Scanner::new();
    let x: i128 = scanner.cin();
    let a: i128 = scanner.cin();
    let d: i128 = scanner.cin();
    let n: i128 = scanner.cin();

    let f = |x: i128| a + (d * (x - 1));

    if n <= 100_000_000 {
        let mut candidates = vec![];
        for i in 1..=n {
            candidates.push((f(i) - x).abs());
        }
        candidates.sort_unstable();
        println!("{}", candidates[0]);
    }
    else {
        let div = 1_000_000;

        let mut vs = vec![];
        for i in 1..=(n / div) {
            let q = i * div;
            if 1 <= q && q <= n {
                vs.push(((f(q) - x).abs(), q));
            }
        }
        vs.sort_unstable();

        let mut candidates = vec![];
        let (_, i) = vs[0];
        let beam = 1_000_000;
        for diff in -beam..=beam {
            let q = i + diff;
            if 1 <= q && q <= n {
                candidates.push((f(q) - x).abs());
            }
        }
        candidates.sort_unstable();
        println!("{}", candidates[0]);
    }
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

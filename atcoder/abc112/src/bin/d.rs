#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    let factors = factors(m);
    for i in (0..factors.len()).rev() {
        let p = factors[i];

        if n * p <= m {
            println!("{}", p);
            return;
        }
    }
}

fn factors(m: usize) -> Vec<usize> {
    let mut p = 1;
    let mut ret = vec![];
    while p * p <= m {
        if m % p == 0 {
            ret.push(p);
            if p != p / m {
                ret.push(m / p);
            }
        }
        p += 1;
    }
    ret.sort_unstable();
    ret
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

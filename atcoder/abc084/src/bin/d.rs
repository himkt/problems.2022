const MAX: usize = 1_000_000;

pub fn eratosthenes_sieve(n: usize) -> Vec<bool> {
    let mut s = vec![true; n];
    s[0] = false;
    s[1] = false;

    for i in 2..n {
        if i * i > n {
            break;
        }
        if s[i] {
            for k in 2..(n + i - 1) / i {
                s[k * i] = false
            }
        }
    }

    s
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let prime_numbers = eratosthenes_sieve(MAX);
    let q: usize = scanner.cin();

    let mut cum = vec![0; MAX + 1];
    for ai in 1..=MAX {
        if ai % 2 == 0 {
            continue;
        }
        let bi = (ai + 1) / 2;
        if prime_numbers[ai] && prime_numbers[bi] {
            cum[ai] += 1;
        }
    }
    for i in 1..=MAX {
        cum[i] += cum[i - 1];
    }
    for _ in 0..q {
        let l: usize = scanner.cin();
        let r: usize = scanner.cin();
        let v = cum[r] - cum[l - 1];
        println!("{}", v);
    }
}

use std::io::Write;
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

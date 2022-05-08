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


pub fn lower_bound(range: std::ops::Range<usize>, prop: &dyn Fn(usize) -> bool) -> usize {
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
fn main() {
    let mut scanner = Scanner::new();
    let n: u128 = scanner.cin();

    let mut primes = vec![];
    let sieve = eratosthenes_sieve(1_000_000);
    for i in 0..sieve.len() {
        if sieve[i] {
            primes.push(i as u128);
        }
    }

    let k = primes.len();
    let mut ans = 0;

    for i in (0..k).rev() {
        let q = primes[i];

        let index = lower_bound(0..i, &|x| {
            let p = primes[x];
            p * q * q * q > n
        });

        ans += index;
    }

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

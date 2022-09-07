const DIV: usize = 1_000_000_007;

#[derive(Debug, Clone)]
pub struct SequentialPrimeFactorization {
    smallest_prime_factors: Vec<usize>,
}

impl SequentialPrimeFactorization {
    pub fn new(n: usize) -> Self {
        let mut smallest_prime_factors: Vec<usize> = (0..=(n + 1)).collect();
        let mut i = 2;

        while i * i <= n {
            if smallest_prime_factors[i] == i {
                let mut j = i;
                while j * i <= n {
                    smallest_prime_factors[j * i] = i;
                    j += 1;
                }
            }

            i += 1;
        }

        Self {
            smallest_prime_factors,
        }
    }

    pub fn factorize(&self, mut x: usize) -> Vec<usize> {
        let mut ret: Vec<usize> = vec![];
        while x != 1 {
            ret.push(self.smallest_prime_factors[x]);
            x /= self.smallest_prime_factors[x];
        }

        ret.sort_unstable();
        ret
    }
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let factorizer = SequentialPrimeFactorization::new(10000);
    let n: usize = scanner.cin();

    let mut cnt = HashMap::new();
    for i in 1..=n {
        for p in factorizer.factorize(i) {
            *cnt.entry(p).or_insert(0) += 1;
        }
    }

    let mut ans: usize = 1;
    for (_, v) in cnt {
        ans *= v + 1;
        ans %= DIV;
    }
    println!("{}", ans);
}

use std::{io::Write, collections::HashMap};
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

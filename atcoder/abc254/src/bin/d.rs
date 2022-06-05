pub fn eratosthenes_sieve_smallest_prime_factors(n: usize) -> Vec<usize> {
    let mut smallest_prime_factors: Vec<usize> = (0..=n + 1).into_iter().collect();
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

    smallest_prime_factors
}

#[derive(Debug, Clone)]
pub struct SequentialPrimeFactorization {
    smallest_prime_factors: Vec<usize>,
}

impl SequentialPrimeFactorization {
    pub fn new(n: usize) -> Self {
        let smallest_prime_factors = eratosthenes_sieve_smallest_prime_factors(n);
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
    let n: usize = scanner.cin();
    let factorizer = SequentialPrimeFactorization::new(2 * 100000);
    let mut cnt = HashMap::new();

    for i in 1..=n {
        let mut prime_factors_by_factor = HashMap::new();
        for factor in factorizer.factorize(i) {
            prime_factors_by_factor.entry(factor).or_insert(0);
            prime_factors_by_factor.entry(factor).and_modify(|v| *v = (*v + 1) % 2);
        }

        let mut k = 1;
        for (p, c) in prime_factors_by_factor {
            if c == 0 { continue; }
            k *= p;
        }

        *cnt.entry(k).or_insert(0) += 1;
    }

    let mut ans = 0;
    for i in 1..=n {
        if cnt.contains_key(&i) {
            ans += cnt[&i] * cnt[&i];
        }
    }

    println!("{}", ans);
}


use std::collections::{VecDeque, HashMap};
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

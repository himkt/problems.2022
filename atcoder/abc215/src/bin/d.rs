pub fn eratosthenes_sieve_smallest_prime_factors(n: usize) -> Vec<usize> {
    let mut smallest_prime_factors: Vec<usize> = (0..=n+1).into_iter().collect();
    let mut i = 2;

    while i*i <= n {
        if smallest_prime_factors[i] == i {
            let mut j = i;
            while j*i <= n {
                smallest_prime_factors[j*i] = i;
                j += 1;
            }
        }

        i += 1;
    }

    smallest_prime_factors
}


#[derive(Debug,Clone)]
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
    let m: usize = scanner.cin();

    let prime_factorizer = SequentialPrimeFactorization::new(100_000);
    let mut btree_set: BTreeSet<usize> = BTreeSet::new();

    for _ in 0..n {
        let a: usize = scanner.cin();
        for x in prime_factorizer.factorize(a) {
            btree_set.insert(x);
        }
    }

    let prime_factors: Vec<usize> = btree_set.into_iter().collect();
    let mut is_coprime: Vec<bool> = vec![true; m+1];

    for i in prime_factors {
        let mut j = 1;
        while i*j <= m {
            is_coprime[i*j] = false;
            j += 1;
        }
    }

    let mut ans: Vec<usize> = vec![];
    for i in 1..=m {
        if is_coprime[i] {
            ans.push(i)
        }
    }

    println!("{}", ans.len());
    for num in ans {
        println!("{}", num);
    }
}


use std::collections::{VecDeque, BTreeSet};
use std::io::{self, Write};
use std::str::FromStr;

#[allow(dead_code)]
struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
#[allow(dead_code)]
impl Scanner {
    fn new() -> Self {
        Self {
            stdin: io::stdin(),
            buffer: VecDeque::new(),
        }
    }

    fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

#[allow(dead_code)]
fn flush() {
    std::io::stdout().flush().unwrap();
}

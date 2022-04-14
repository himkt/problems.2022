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
    let a: Vec<usize> = scanner.vec(n);

    let factorizer = SequentialPrimeFactorization::new(1_000_000);

    let mut factors = vec![HashSet::new(); n];
    let mut all_factors = vec![];

    for (i, &ai) in a.iter().enumerate() {
        let fs: HashSet<usize> = factorizer.factorize(ai).into_iter().collect();
        factors[i] = fs.clone();
        all_factors.extend(fs)
    }

    let num_factors = all_factors.len();
    let all_factor_set: HashSet<usize> = all_factors.into_iter().collect();
    let num_uniq_factors = all_factor_set.len();

    if num_factors == num_uniq_factors {
        println!("pairwise coprime");
        return;
    }

    let mut intersection: HashSet<usize> = factors[0]
        .intersection(&factors[1])
        .cloned()
        .collect();

    for i in 2..n {
        intersection = intersection.intersection(&factors[i])
            .cloned()
            .collect();
    }

    if intersection.is_empty() {
        println!("setwise coprime");
    }
    else {
        println!("not coprime");
    }
}


use std::collections::{VecDeque, HashSet};
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
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
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

#[macro_export]
macro_rules! trace {
    ($x:expr) => {
        #[cfg(debug_assertions)]
        eprintln!(">>> {} = {:?}", stringify!($x), $x)
    };
    ($($xs:expr),*) => { trace!(($($xs),*)) }
}

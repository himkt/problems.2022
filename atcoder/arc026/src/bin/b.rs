fn factorize(n: usize) -> Vec<usize> {
    let mut ret: HashSet<usize> = HashSet::new();
    let limit = (n as f64).sqrt().ceil() as usize;
    for p in 1..=limit {
        if n % p == 0 {
            ret.insert(p);
            ret.insert(n / p);
        }
    }
    ret.into_iter().collect::<Vec<usize>>()
}


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let factors = factorize(n);
    let sum: usize = factors.iter().sum::<usize>() - n;

    match sum.cmp(&n) {
        Ordering::Equal => println!("Perfect"),
        Ordering::Less => println!("Deficient"),
        Ordering::Greater => println!("Abundant"),
    }
}


use std::cmp::Ordering;
use std::collections::{VecDeque, HashSet};
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

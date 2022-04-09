fn factors(n: usize) -> Vec<usize> {
    let k = (n as f64).sqrt() as usize;

    let mut factors: BTreeSet<usize> = BTreeSet::new();
    for p in 1..=k {
        if n % p == 0 {
            factors.insert(p);
            factors.insert(n / p);
        }
    }

    factors.into_iter().collect()
}


fn main() {
    let mut scanner = Scanner::new();

    let n: usize = scanner.cin();

    let mut ans = 0;
    for factor in factors(n) {
        let k = factor;
        let m = (n / k) - 1;

        if m == 0 {
            continue;
        }

        if n / m == n % m {
            ans += m;
        }
    }

    println!("{}", ans);
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

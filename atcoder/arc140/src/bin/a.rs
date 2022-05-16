#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();

    for q in 1..=n {
        if n % q != 0 {
            continue;
        }

        let mut counters = vec![HashMap::new(); q];
        for i in 0..n {
            *counters[i % q].entry(s[i]).or_insert(0) += 1;
        }

        let mut cost = 0;
        for i in 0..q {
            let vs = &counters[i];
            let maxv: usize = *vs.values().max().unwrap();
            let sumv: usize = vs.values().sum();
            cost += sumv - maxv;
        }

        if cost <= k {
            println!("{}", q);
            return;
        }
    }
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

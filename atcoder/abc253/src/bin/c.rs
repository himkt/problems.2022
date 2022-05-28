#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let q: usize = scanner.cin();

    let mut counter = BTreeMap::new();

    for _ in 0..q {
        let t: usize = scanner.cin();
        if t == 1 {
            let x: i64 = scanner.cin();
            *counter.entry(x).or_insert(0) += 1;
        }
        else if t == 2 {
            let x: i64 = scanner.cin();
            let c: usize = scanner.cin();

            if !counter.contains_key(&x) {
                continue;
            }

            counter.entry(x).and_modify(|e| {
                let v = *e;

                let new_cnt = if v < c {
                    0
                }
                else {
                    v - c
                };

                *e = new_cnt;
            });

            if counter[&x] == 0 {
                counter.remove(&x);
            }
        }
        else if t == 3 {
            let min = counter.keys().next().unwrap();
            let max = counter.keys().last().unwrap();
            println!("{}", max - min);
        }
    }
}


use std::collections::{VecDeque, BTreeMap};
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

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();

    let a: Vec<usize> = scanner.vec(n);
    let b: Vec<usize> = scanner.vec(k);

    let mut c: Vec<(usize, usize)> = (0..n).map(|i| (a[i], i+1)).collect();
    c.sort_unstable_by_key(|&(v, _)| Reverse(v));
    // println!("{:?}", c);

    let max = c[0].0;
    let mut possible = false;

    for i in 0..n {
        if c[i].0 != max {
            continue;
        }
        if b.contains(&c[i].1) {
            possible = true;
        }
    }

    if possible {
        println!("Yes");
    }
    else {
        println!("No");
    }
}


use std::cmp::Reverse;
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

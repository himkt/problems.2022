#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let w: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut good_numbers = HashSet::new();

    for i in 0..n {
        if a[i] <= w {
            good_numbers.insert(a[i]);
        }
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if a[i] + a[j] <= w {
                good_numbers.insert(a[i] + a[j]);
            }
        }
    }


    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if i ==j || j == k || k == i {
                    continue;
                }
                if a[i] + a[j] + a[k] <= w {
                    good_numbers.insert(a[i] + a[j] + a[k]);
                }
            }
        }
    }

    let ans = good_numbers.len();
    println!("{}", ans);
}


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
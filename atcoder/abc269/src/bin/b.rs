#[allow(clippy::needless_range_loop, clippy::collapsible_if)]
fn check(a: usize, b: usize, c: usize, d: usize, s: &[Vec<char>]) -> bool {
    let mut t = vec![
        "..........".chars().collect::<Vec::<char>>(); 10
    ];

    for i in 0..10 {
        for j in 0..10 {
            if i >= a && i <= b {
                if j >= c && j <= d {
                    t[i][j] = '#';
                }
            }
        }
    }

    for i in 0..10 {
        for j in 0..10 {
            if s[i][j] != t[i][j] {
                return false;
            }
        }
    }

    true
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let s: Vec<Vec<char>> = (0..10)
        .map(|_| scanner.cin::<String>().chars().collect())
        .collect();

    for a in 0..10 {
        for b in a..10 {
            for c in 0..10 {
                for d in c..10 {
                    if check(a, b, c, d, &s) {
                        println!("{} {}", a + 1, b + 1);
                        println!("{} {}", c + 1, d + 1);
                    }
                }
            }
        }
    }
}

use std::io::Write;
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

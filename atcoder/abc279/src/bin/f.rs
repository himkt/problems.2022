#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let q: usize = scanner.cin();
    let mut hsh: HashMap<usize, usize> = HashMap::new();
    for i in 1..=n {
        hsh.insert(i, 0);
    }
    let mut cnt = 0;
    for _ in 0..q {
        let optype: usize = scanner.cin();
        match optype {
            1 => {
                let x: usize = scanner.cin();
                let y: usize = scanner.cin();
                let nc = hsh[&y];
                hsh.entry(x).and_modify(|e| *e += nc);
                hsh.entry(y).and_modify(|e| *e -= nc);
            }
            2 => {
                cnt += 1;
                *hsh.entry(cnt).or_insert(0) += 1;
            }
            3 => {
                println!("{}")
            }
            _ => {}
        }
    }
}

use std::{io::Write, collections::HashMap};
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

#[macro_export]
macro_rules! debug {
    () => {
        #[cfg(debug_assertions)]
        println!();
    };
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}

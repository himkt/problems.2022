#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let a: usize = scanner.cin();
    let b: usize = scanner.cin();
    let c: usize = scanner.cin();
    let d: usize = scanner.cin();
    let e: usize = scanner.cin();
    let mut set = HashSet::new();
    set.insert(a);
    set.insert(b);
    set.insert(c);
    set.insert(d);
    set.insert(e);
    println!("{}", set.len());
}

use std::{io::Write, collections::HashSet};
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
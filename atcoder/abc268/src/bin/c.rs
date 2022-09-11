#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let p: Vec<usize> = scanner.vec(n);

    let mut score_by_rotation = HashMap::new();
    for i in 0..n {
        // 料理 p[i] が (i - 1) % N, i (i + 1) % N に移動するための
        // rotation 回数 q を求める
        let q = (p[i] + n) - i;
        *score_by_rotation.entry((q - 1) % n).or_insert(0) += 1;
        *score_by_rotation.entry(q % n).or_insert(0) += 1;
        *score_by_rotation.entry((q + 1) % n).or_insert(0) += 1;
    }

    let ans = score_by_rotation.values().max().unwrap();
    println!("{}", ans);
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

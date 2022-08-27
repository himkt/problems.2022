#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let c: usize = scanner.cin();
    let k: usize = scanner.cin();
    let mut t: Vec<usize> = scanner.vec(n);
    t.sort_unstable();

    let mut cnt = HashMap::new();
    t.iter().for_each(|&x| {
        *cnt.entry(x).or_insert(0) += 1;
    });


    let mut cur = 0;
    let mut ans = 0;
    while cur < n {
        let ti = t[cur] + k;
        let pi = (cnt[&t[cur]] + c - 1) / c;
        ans += pi;

        let frozen_cur = cur;
        for d in 0..(pi * c) {
            if ti < t[frozen_cur + d] {
                break;
            }
            cur += 1;
            if cur == n {
                break;
            }
        }
    }

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

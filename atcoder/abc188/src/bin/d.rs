#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let c: i64 = scanner.cin();

    let mut tot: HashMap<usize, i64> = HashMap::new();
    for _ in 0..n {
        let a: usize = scanner.cin();
        let b: usize = scanner.cin();
        let c: i64 = scanner.cin();

        *tot.entry(a - 1).or_insert(0) += c;
        *tot.entry(b).or_insert(0) -= c;
    }

    let mut keys: Vec<usize> = tot.keys().cloned().collect();
    keys.sort_unstable();

    for i in 1..keys.len() {
        let prv = tot[&keys[i-1]];
        tot.entry(keys[i]).and_modify(|e| *e += prv);
    }

    let mut ans = 0;
    for i in 0..(keys.len() - 1) {
        let duration = (keys[i + 1] - keys[i]) as i64;
        let ci = tot[&keys[i]];
        ans += duration * c.min(ci);
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

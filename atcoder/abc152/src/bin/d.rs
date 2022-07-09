#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let mut cnt = HashMap::new();
    for i in 1..=n {
        let d = (i as f64).log10().floor() as u32;
        let head = i / 10usize.pow(d);
        let tail = i % 10;
        *cnt.entry((head, tail)).or_insert(0) += 1;
    }

    let mut ans = 0;
    for i in 1..=n {
        let d = (i as f64).log10().floor() as u32;
        let head = i / 10usize.pow(d);
        let tail = i % 10;
        if cnt.contains_key(&(tail, head)) {
            ans += cnt[&(tail, head)];
        }
    }

    println!("{}", ans);
}

use std::{io::Write, collections::HashMap}; pub fn flush() { std::io::stdout().flush().unwrap(); }
pub struct Scanner { buffer: std::collections::VecDeque<String>, buf: String }
impl Scanner {
    pub fn new() -> Self { Scanner { buffer: std::collections::VecDeque::new(), buf: String::new() } }
    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        if !self.buffer.is_empty() { return self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap(); }
        self.buf.truncate(0); std::io::stdin().read_line(&mut self.buf).ok();
        self.buf.to_owned().split_whitespace().for_each(|x| self.buffer.push_back(String::from(x)));
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> { (0..n).map(|_| self.cin()).collect() }
}
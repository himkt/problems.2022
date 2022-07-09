#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut a: Vec<usize> = scanner.vec(n);
    a.sort_unstable();

    let mut pairs: Vec<(usize, usize)> = (0..m)
        .map(|_| {
            let b: usize = scanner.cin();
            let c: usize = scanner.cin();
            (b, c)
        })
        .collect();
    pairs.sort_unstable_by_key(|&(_, c)| Reverse(c));

    let mut cursor = 0;
    for i in 0..n {
        if pairs[cursor].1 > a[i] {
            a[i] = pairs[cursor].1;
            pairs[cursor].0 -= 1;
        }

        if pairs[cursor].0 == 0 {
            cursor += 1;
        }

        if cursor == m {
            break;
        }
    }

    let ans: usize = a.iter().sum();
    println!("{}", ans);
}

use std::{io::Write, cmp::Reverse}; pub fn flush() { std::io::stdout().flush().unwrap(); }
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

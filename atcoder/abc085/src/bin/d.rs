#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let h: usize = scanner.cin();
    let mut strengthes = vec![];
    let mut amax = 0;
    for _ in 0..n {
        let a: usize = scanner.cin();
        let b: usize = scanner.cin();
        strengthes.push((a, b));
        amax = amax.max(a);
    }
    strengthes.sort_unstable_by_key(|&(_, bi)| Reverse(bi));

    let mut ans = 0;
    let mut cur = 0;

    for i in 0..n {
        let (_, bi) = strengthes[i];
        if bi >= amax {
            cur += bi;
            ans += 1;
        }
        if cur >= h {
            println!("{}", ans);
            return;
        }
    }

    strengthes.sort_unstable_by_key(|&(ai, _)| Reverse(ai));
    let hrem = h - cur;
    let crem = (hrem + amax - 1) / amax;
    ans += crem;

    println!("{}", ans);
}

use std::{io::Write, cmp::Reverse};
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

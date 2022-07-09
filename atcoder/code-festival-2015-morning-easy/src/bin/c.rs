pub fn lower_bound(range: std::ops::Range<usize>, prop: &dyn Fn(usize) -> bool) -> usize {
    if prop(range.start) {
        return range.start;
    }

    let mut ng = range.start;
    let mut ok = range.end;

    while ok - ng > 1 {
        let middle = ng + (ok - ng) / 2;
        match prop(middle) {
            true => ok = middle,
            false => ng = middle,
        }
    }

    ok
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();
    let m: usize = scanner.cin();
    let r: usize = scanner.cin();

    let mut s: Vec<usize> = (0..n - 1)
        .map(|_| {
            scanner.cin::<usize>()
        })
        .collect();

    s.sort_unstable_by_key(|&v| Reverse(v));

    if k < n {
        let sum: usize = s.iter().take(k).cloned().sum();
        if sum >= k * r {
            println!("0");
            return;
        }
    }

    let sum: usize = s.iter().take(k - 1).cloned().sum();

    let li = lower_bound(
        0..(m + 1),
        &|x| (sum + x) / k >= r,
    );

    if li == m + 1 {
        println!("-1");
    }
    else {
        println!("{}", li);
    }
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

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

const INF: usize = 1_000_000_000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);
    let b: Vec<usize> = scanner.vec(n);

    let mut positions = HashMap::new();
    for (i, &bi) in b.iter().enumerate() {
        positions.insert(bi, i);
    }

    let mut pairs = vec![];
    for (i, &ai) in a.iter().enumerate() {
        let mut k = 1;
        while k * ai <= n {
            let j = positions[&(k * ai)];
            pairs.push((i, j));
            k += 1;
        }
    }

    pairs.sort_unstable_by_key(|&(i, j)| (i, Reverse(j)));

    let mut lis = vec![INF; n];
    let m: usize = pairs.len();
    for i in 0..m {
        let j = lower_bound(0..n, &|x| lis[x] >= pairs[i].1);
        lis[j] = pairs[i].1;
    }

    let ans: usize = lower_bound(0..n, &|x| lis[x] >= INF);
    println!("{}", ans);
}

use std::{io::Write, collections::HashMap, cmp::Reverse}; pub fn flush() { std::io::stdout().flush().unwrap(); }
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

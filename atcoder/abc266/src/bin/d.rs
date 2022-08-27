#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let pairs: Vec<(usize, usize, usize)> = (0..n)
        .map(|_| {
            let t: usize = scanner.cin();
            let x: usize = scanner.cin();
            let a: usize = scanner.cin();
            (t, x, a)
        })
        .collect();
    let tmax = pairs.last().unwrap().0;

    let mut points = HashMap::new();
    for (t, x, a) in pairs {
        points.entry((t, x)).or_insert(a);
    }

    let mut dp = vec![vec![0; 5]; tmax + 1];
    for ti in 1..=tmax {
        let mut row = vec![0; 5];
        for x in 0..5 {
            if ti < x {
                continue;
            }
            let bonus = if points.contains_key(&(ti, x)) {
                points[&(ti, x)]
            }
            else {
                0
            };
            if x > 0 {
                row[x] = row[x].max(dp[ti - 1][x - 1] + bonus);  // 左から
            }
            row[x] = row[x].max(dp[ti - 1][x] + bonus);  // stay
            if x < 4 {
                row[x] = row[x].max(dp[ti - 1][x + 1] + bonus);  // 右から
            }
        }
        // println!("ti={}, row={:?}", ti, row);
        dp[ti] = row;
    }

    // println!("{:?}", dp);
    println!("{}", dp.last().unwrap().iter().max().unwrap());
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

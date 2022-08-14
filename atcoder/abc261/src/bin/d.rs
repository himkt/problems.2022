const LIMIT: usize = 5000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let x: Vec<usize> = scanner.vec(n);
    let bonus: HashMap<usize, usize> = (0..m)
        .map(|_| {
            let k: usize = scanner.cin();
            let v: usize = scanner.cin();
            (k, v)
        })
        .collect();

    let mut dp = vec![vec![0; LIMIT + 1]; n];

    let mut score = x[0];
    if bonus.contains_key(&1) {
        score += bonus[&1];
    }
    dp[0][1] = score;
    let mut mx = score;

    for i in 1..n {
        dp[i][0] = mx;
        mx = 0;

        for j in 0..LIMIT {
            if j > 0 && dp[i - 1][j] == 0 {
                continue;
            }

            let k = j + 1;
            let mut score = x[i];
            if bonus.contains_key(&k) {
                score += bonus[&k];
            }

            //dp[i][k] = dp[i - 1][k].max(dp[i - 1][j] + score);
            dp[i][k] = dp[i - 1][j] + score;
            mx = mx.max(dp[i - 1][j] + score);
        }
    }

    let ans = dp[n - 1].iter().max().unwrap();
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

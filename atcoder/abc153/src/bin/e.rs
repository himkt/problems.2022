const INF: usize = 1_000_000_000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let n: usize = scanner.cin();

    let mut a: Vec<usize> = vec![0; n];
    let mut b: Vec<usize> = vec![0; n];
    (0..n).for_each(|i| {
        let ai: usize = scanner.cin();
        let bi: usize = scanner.cin();
        a[i] = ai;
        b[i] = bi;
    });

    fn minus_or_zero(u: usize, v: usize) -> usize {
        if u >= v { u - v } else { 0 }
    }

    let mut dp = vec![vec![INF; h + 1]; n + 1];
    dp[0][h] = 0;

    for i in 0..n {
        for j in (0..=h).rev() {
            dp[i + 1][j] = dp[i + 1][j].min(dp[i][j]);

            let nj = minus_or_zero(j, a[i]);
            dp[i + 1][nj] = dp[i + 1][nj].min(dp[i + 1][j] + b[i]);
        }
    }

    println!("{}", dp[n][0]);
}

use std::io::Write;
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

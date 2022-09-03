const MIN: i64 = - 1_000_000_000_000_000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut a = vec![0; n + 1];
    for i in 1..=n {
        a[i] = scanner.cin::<i64>();
    }

    let mut dp = vec![
        vec![MIN; m + 1]; n + 1
    ];
    for i in 1..=n {
        dp[i][1] = a[i];
    }

    for i in 2..=n {
        for j in 1..=m {
            dp[i][j] = dp[i][j]
                .max(dp[i - 1][j])
                .max(dp[i - 1][j - 1] + (j as i64) * a[i]);
        }
    }

    let mut ans = dp[1][m];
    for i in 2..=n {
        ans = ans.max(dp[i][m]);
    }
    println!("{}", ans);
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

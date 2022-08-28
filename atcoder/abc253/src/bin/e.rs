const DIV: usize = 998_244_353;

fn lower(j: usize, k: usize) -> Option<usize> {
    if j < k {
        return None;
    }
    Some(j - k)
}

fn upper(j: usize, k: usize, m: usize) -> Option<usize> {
    if j + k > m {
        return None;
    }
    Some(j + k)
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let k: usize = scanner.cin();

    let mut dp = vec![vec![0; m + 1]; n + 1];
    for j in 1..=m {
        dp[1][j] = 1;
    }

    for i in 2..=n {
        let mut cumsum = vec![0; m + 1];
        for k in 1..=m {
            cumsum[k] += dp[i - 1][k] + cumsum[k - 1];
            cumsum[k] %= DIV;
        }

        for j in 1..=m {
            if k == 0 {
                dp[i][j] += cumsum[m];
                dp[i][j] %= DIV;
                continue;
            }
            if let Some(min) = lower(j, k) {
                dp[i][j] += cumsum[min];
                dp[i][j] %= DIV;
            }
            if let Some(max) = upper(j, k, m) {
                dp[i][j] += DIV + cumsum[m] - cumsum[max - 1];
                dp[i][j] %= DIV;
            }
        }
    }

    let mut ans = 0;
    for v in dp[n].iter() {
        ans += v;
        ans %= DIV;
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

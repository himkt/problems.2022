#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let r: Vec<i64> = scanner.vec(n);

    let mut dp = vec![vec![0; 2]; n];

    // 0: increasing, 1: decreasing
    dp[0][0] = 1;
    dp[0][1] = 1;

    let mut ans = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            if r[i] < r[j] {
                dp[j][0] = dp[j][0].max(dp[i][1] + 1);
            }
            if r[i] > r[j] {
                dp[j][1] = dp[j][1].max(dp[i][0] + 1)
            }
        }
        ans = ans.max(dp[i][0]).max(dp[i][1]);
    }

    if ans < 3 {
        println!("0");
    }
    else {
        println!("{}", ans);
    }
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

const DIV: usize = 998244353;
const MAX: usize = 61;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    if n > 60 {
        println!("0");
        return;
    }

    let mut sizes = vec![0; MAX];
    for j in 1..MAX {
        if (1 << j) - 1 < m {
            sizes[j] = (1 << j) - (1 << (j - 1));
            sizes[j] %= DIV;
        }
        else {
            sizes[j] = m - (1 << (j - 1)) + 1;
            sizes[j] %= DIV;
            break;
        }
    }

    let mut dp = vec![vec![0; MAX]; MAX];
    dp[0] = sizes.clone();

    for i in 1..n {
        for j in 1..MAX {
            let mut ret = 0;
            for k in 1..j {
                ret += dp[i - 1][k];
                ret %= DIV;
            }
            dp[i][j] = sizes[j] * ret;
            dp[i][j] %= DIV;
        }
    }

    let mut ans = 0;
    for v in dp[n - 1].iter() {
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

const DIV: usize = 998244353;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut ans = 0;

    for div in 1..=n {
        // dp[i][j][k]
        // :=
        // i 番目までの要素を使って作った長さ j の部分列のうち
        // 和を div で割ったあまりが k であるものの数
        // i [0, n]
        // j [0, n]
        // k [0, n - 1]
        let mut dp = vec![vec![vec![0; div]; n + 1]; n + 1];
        dp[0][0][0] = 1;

        for i in 1..=n {
            let ai = a[i - 1];
            for j in 0..=n {
                for k in 0..div {
                    debug!("i={}, j={}, k={}", i, j, k);
                    dp[i][j][k] += dp[i - 1][j][k];
                    dp[i][j][k] %= DIV;

                    let l = (k + ai) % div;
                    if j == n {
                        continue;
                    }
                    dp[i][j + 1][l] += dp[i - 1][j][k];
                    dp[i][j + 1][l] %= DIV;
                }
            }
        }

        debug!("div={}, dp={:?}", div, dp[n][div][0]);
        for row in dp[n].iter() {
            debug!("{:?}", row);
        }
        ans += dp[n][div][0];
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

#[macro_export]
macro_rules! debug {
    () => {
        #[cfg(debug_assertions)]
        println!();
    };
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}

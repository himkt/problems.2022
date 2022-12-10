#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();
    let d: usize = scanner.cin();
    let a: Vec<i64> = scanner.vec(n);

    let mut dp: Vec<Vec<Vec<i64>>> = ndarray!(-1; n + 1, k + 1, d);
    dp[0][0][0] = 0;

    for i in 0..n {
        for l in 0..=k {
            for m in 0..d {
                dp[i + 1][l][m] = dp[i + 1][l][m].max(dp[i][l][m]);

                if dp[i][l][m] == -1 {
                    continue;
                }
                if l == k {
                    continue;
                }

                let p = (m + a[i] as usize) % d;
                let nv = dp[i][l][m] + a[i];
                dp[i + 1][l + 1][p] = dp[i + 1][l + 1][p].max(nv);
            }
        }
    }

    println!("{}", dp[n][k][0]);
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

#[macro_export]
macro_rules! ndarray {
    // ndarray!(val; *shape)
    ($x:expr;) => { $x };
    ($x:expr; $size:expr $( , $rest:expr )*) => {
        vec![ndarray!($x; $($rest),*); $size]
    };
}

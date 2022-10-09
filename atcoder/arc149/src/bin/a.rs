#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: i128 = scanner.cin();

    let mut dp: Vec<Vec<i128>> = vec![vec![-1; 10]; n + 1];
    for p in 1..=9 {
        dp[1][p] = (p as i128) % m;
    }

    for k in 2..=n {
        for p in 1..=9 {
            dp[k][p] = (dp[k - 1][p] * 10 + p as i128) % m;
        }
    }

    for i in (1..=n).rev() {
        for p in (1..=9).rev() {
            if dp[i][p] == 0 {
                let mut ans = "".to_string();
                let c = &p.to_string();
                for _ in 0..i {
                    ans += c;
                }
                println!("{}", ans);
                return;
            }
        }
    }

    println!("-1");
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

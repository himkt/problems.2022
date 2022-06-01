const DIV: usize = 998244353;


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    if n > 60 {
        println!("0");
        return;
    }

    let mut dp = vec![vec![0; 61]; n];
    dp[0][1] = 1;

    let mut p = 2;  // p=2^(k-1)
    for k in 2..61 {
        if p > m {
            dp[0][k] = 0;
        }
        else {
            let q = 2 * p;  // q=2^k
            dp[0][k] = (q - 1).min(m) - p + 1;
            dp[0][k] %= DIV;
        }
        p *= 2;
    }

    for i in 1..n {
        for k in 1..61 {
            for j in 1..k {
                dp[i][k] += dp[0][k] * dp[i-1][j];
                dp[i][k] %= DIV;
            }
        }
    }

    let mut ans = 0;
    for j in 0..61 {
        ans += dp[n-1][j];
        ans %= DIV;
    }

    println!("{}", ans);
}


use std::collections::VecDeque;
use std::io::{self, Write};
use std::str::FromStr;

pub struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
impl Scanner {
    fn new() -> Self {
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
    }

    pub fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

pub fn flush() {
    std::io::stdout().flush().unwrap();
}

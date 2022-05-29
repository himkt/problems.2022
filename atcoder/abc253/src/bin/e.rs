const DIV: usize = 998244353;


pub fn modpow(mut a: usize, mut n: usize, m: usize) -> usize {
    let mut ans = 1;

    while n > 0 {
        if n & 1 == 1 {
            ans = ans * a % m;
        }

        a = a * a % m;
        n >>= 1;
    }

    ans
}


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();

    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let k: usize = scanner.cin();

    if k == 0 {
        let ans = modpow(m, n, DIV);
        println!("{}", ans);
        return;
    }

    let mut dp = vec![vec![0; m]; n];
    for j in 0..m {
        dp[0][j] = 1;
    }

    for i in 1..n {
        let mut cum = vec![0; m];
        cum[0] = dp[i-1][0];
        for j in 1..m {
            cum[j] = dp[i-1][j] + cum[j-1];
        }

        for j in 0..m {
            let mut v = cum[m-1];

            let r = (j + k - 1).min(m - 1);
            v -= cum[r];

            if j >= k {
                let l = j - k;
                v += cum[l];
            }

            dp[i][j] = v % DIV;
        }
    }

    let mut ans = 0;
    for &v in dp[n-1].iter() {
        ans += v;
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

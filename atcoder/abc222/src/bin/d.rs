const DIV: usize = 998244353;
const MAX: usize = 3000;


fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);
    let b: Vec<usize> = scanner.vec(n);

    let mut dp: Vec<Vec<usize>> = vec![vec![0; MAX+2]; n];
    for i in a[0]..=b[0] {
        dp[0][i] = 1;
    }

    for t in 1..n {

        for i in 0..=MAX {
            dp[t-1][i+1] += dp[t-1][i];
        }

        for i in a[t]..=b[t] {
            dp[t][i] += dp[t-1][i];
            dp[t][i] %= DIV;
        }
    }

    let mut ans: usize = 0;
    for v in dp[n-1].iter() {
        ans += v;
        ans %= DIV;
    }

    println!("{}", ans);
}


use std::collections::VecDeque;
use std::io::{self, Write};
use std::str::FromStr;

#[allow(dead_code)]
struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
#[allow(dead_code)]
impl Scanner {
    fn new() -> Self {
        Self {
            stdin: io::stdin(),
            buffer: VecDeque::new(),
        }
    }

    fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

#[allow(dead_code)]
fn flush() {
    std::io::stdout().flush().unwrap();
}

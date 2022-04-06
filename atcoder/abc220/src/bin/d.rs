const K: usize = 10;
const DIV: usize = 998_244_353;

fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut dp: Vec<Vec<usize>> = vec![vec![0; K]; n-1];

    dp[0][(a[1]+a[0]) % K] = 1;
    dp[0][(a[1]*a[0]) % K] += 1;

    for i in 1..n-1 {
        for k in 0..K {
            dp[i][(a[i+1]+k) % K] += dp[i-1][k];
            dp[i][(a[i+1]+k) % K] %= DIV;
            dp[i][(a[i+1]*k) % K] += dp[i-1][k];
            dp[i][(a[i+1]*k) % K] %= DIV;
        }
    }

    let ans: Vec<_> = dp
        .last()
        .unwrap()
        .iter()
        .map(|x| x.to_string())
        .collect();

    println!("{}", ans.join(" "));
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

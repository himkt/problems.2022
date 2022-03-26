const DIV: usize = 998244353;


fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let mut dp: Vec<Vec<usize>> = vec![vec![0; 9]; n];
    for i in 0..9 {
        dp[0][i] = 1;
    }

    for t in 0..n-1 {
        for i in 0..9 {
            if i >= 1 {
                dp[t+1][i] += dp[t][i-1];
                dp[t+1][i] %= DIV;
            };

            dp[t+1][i] += dp[t][i];
            dp[t+1][i] %= DIV;

            if i <= 7 {
                dp[t+1][i] += dp[t][i+1];
                dp[t+1][i] %= DIV;
            };
        }
    }

    let mut ans: usize = dp[dp.len()-1].iter().sum();
    ans %= DIV;
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

    fn usize1(&mut self) -> usize {
        self.cin::<usize>() - 1
    }

    fn chars(&mut self) -> Vec<char> {
        self.cin::<String>().chars().collect()
    }

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

#[allow(dead_code)]
fn flush() {
    std::io::stdout().flush().unwrap();
}

#[allow(clippy::collapsible_if)]
#[allow(clippy::collapsible_else_if)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let k: i64 = scanner.cin();
    let a: Vec<i64> = scanner.vec(n);
    let b: Vec<i64> = scanner.vec(n);

    let mut dp = vec![vec![0, 0]; n];
    dp[0][0] = 1;
    dp[0][1] = 1;

    for i in 1..n {
        if dp[i-1][0] != 0 {
            if (a[i-1] - a[i]).abs() <= k {
                if dp[i][0] == 0 {
                    dp[i][0] = 1;
                }
            }
            if (a[i-1] - b[i]).abs() <= k {
                if dp[i][1] == 0 {
                    dp[i][1] = 1;
                }
            }
        }

        if dp[i-1][1] != 0 {
            if (b[i-1] - a[i]).abs() <= k {
                if dp[i][0] == 0 {
                    dp[i][0] = 1;
                }
            }
            if (b[i-1] - b[i]).abs() <= k {
                if dp[i][1] == 0 {
                    dp[i][1] = 1;
                }
            }
        }
    }

    let dpl = &dp[dp.len()-1];
    if dpl[0] == 1 || dpl[1] == 1 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}


use std::collections::{VecDeque};
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

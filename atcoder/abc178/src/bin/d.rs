const DIV: usize = 1_000_000_000 + 7;

fn main() {
    let mut scanner = Scanner::new();
    let s: usize = scanner.cin();

    let mut dp = vec![vec![0; s + 1]; s + 1];
    for j in 3..=s {
        dp[1][j] = 1;
    }

    let ss = s / 3;
    for i in 2..=ss {
        if s < 3 * (i - 1) {
            continue;
        }
        let l = 3 * (i - 1);
        for j in l..=s {
            for k in 3..=s {
                if j + k > s {
                    break;
                }
                dp[i][j + k] += dp[i - 1][j];
                dp[i][j + k] %= DIV;
            }
        }
    }

    let mut ans = 0;
    for row in dp {
        ans += row[s];
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
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
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

#[macro_export]
macro_rules! trace {
    ($x:expr) => {
        #[cfg(debug_assertions)]
        eprintln!(">>> {} = {:?}", stringify!($x), $x)
    };
    ($($xs:expr),*) => { trace!(($($xs),*)) }
}

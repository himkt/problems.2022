const DIV: usize = 998244353;


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let k: usize = scanner.cin();

    let mut dp: Vec<Vec<usize>> = vec![vec![0; k+1]; n];

    for j in 1..=m {
        dp[0][j] = 1;
    }

    // println!("{:?}", dp);

    for i in 1..n {
        for v in 1..=k {
            for j in 1..=m {
                // println!("i={}, j={}, p={}", i, j, p);
                if v + j > k {
                    continue;
                }
                if dp[i-1][v] == 0 {
                    continue;
                }
                dp[i][v+j]  = dp[i-1][v] + dp[i][v+j];
                dp[i][v+j] %= DIV;
            }
        }
    }

    // println!("{:?}", dp);
    let ans: usize = dp[dp.len()-1].iter().sum();
    let ans = ans % DIV;
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

const INF: i64 = 1e18 as i64;


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();
    let c: i64 = scanner.cin();

    let mut a: Vec<Vec<i64>> = vec![vec![0; w]; h];
    for i in 0..h {
        let ai: Vec<i64> = scanner.vec(w);
        a[i] = ai;
    }

    let mut ans: i64 = INF;

    for _ in 0..2 {
        let mut dp: Vec<Vec<i64>> = vec![vec![INF; w]; h];
        for i in 0..h {
            for j in 0..w {
                if i > 0 {
                    dp[i][j] = dp[i][j].min(dp[i-1][j]);
                }
                if j > 0 {
                    dp[i][j] = dp[i][j].min(dp[i][j-1]);
                }

                let i_i64 = i as i64;
                let j_i64 = j as i64;

                ans = ans.min(a[i][j]+(i_i64+j_i64)*c+dp[i][j]);
                dp[i][j] = ans.min(dp[i][j].min(a[i][j]-(i_i64+j_i64)*c));
            }
        }

        // (a, b) -> (c, d)
        // iter0: (a, b) > (c, d)
        // iter1: (c, d) > (a, b)
        a.reverse();
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

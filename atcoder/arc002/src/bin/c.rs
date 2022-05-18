const INF: usize = 1001001001001;


#[allow(clippy::needless_range_loop)]
#[allow(clippy::if_same_then_else)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let cs: Vec<char> = scanner.cin::<String>().chars().collect();
    let cs = [vec!['-'], cs].concat();

    let chars = vec!['A', 'B', 'X', 'Y'];

    fn is_shortcut(cs: &[char], chars: &[char], t: usize, i: usize, j: usize) -> bool {
        cs[t - 1] == chars[i] && cs[t] == chars[j]
    }

    let mut ans = n;
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                for m in 0..4 {

                    let mut dp: Vec<usize> = vec![INF; n+1];
                    dp[0] = 0;
                    dp[1] = 1;

                    for t in 2..=n {
                        if is_shortcut(&cs, &chars, t, i, j) || is_shortcut(&cs, &chars, t, k, m) {
                            dp[t] = dp[t - 2].min(dp[t - 1]) + 1;
                        }
                        else {
                            dp[t] = dp[t-1] + 1;
                        }
                    }

                    ans = ans.min(dp[n]);
                }
            }
        }
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

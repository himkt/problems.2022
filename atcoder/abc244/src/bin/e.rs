const DIV: usize = 998_244_353;


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let k: usize = scanner.cin();
    let s: usize = scanner.cin();
    let t: usize = scanner.cin();
    let x: usize = scanner.cin();

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..m {
        let u: usize = scanner.cin();
        let v: usize = scanner.cin();
        graph[u-1].push(v-1);
        graph[v-1].push(u-1);
    }

    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; 2]; n]; k+1];
    // dp[t][i][k]
    // t: 時間
    // i: ノードID
    // k: 偶奇
    dp[0][s-1][0] = 1;

    for t in 1..=k {
        for i in 0..n {
            for &j in graph[i].iter() {
                // i -> j のパス
                if j == x - 1 {
                    dp[t][j][0] += dp[t-1][i][1];
                    dp[t][j][1] += dp[t-1][i][0];
                }
                else {
                    dp[t][j][0] += dp[t-1][i][0];
                    dp[t][j][1] += dp[t-1][i][1];
                }

                dp[t][j][0] %= DIV;
                dp[t][j][1] %= DIV;
            }
        }
    }

    let ans = dp[k][t-1][0] % DIV;
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

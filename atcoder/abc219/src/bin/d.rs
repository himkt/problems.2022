const INF: usize = 1_000_000_000;


fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let x: usize = scanner.cin();
    let y: usize = scanner.cin();

    let mut a: Vec<usize> = vec![0; n];
    let mut b: Vec<usize> = vec![0; n];
    for i in 0..n {
        let ai: usize = scanner.cin();
        let bi: usize = scanner.cin();
        a[i] = ai;
        b[i] = bi;
    }

    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![INF; y+1]; x+1]; n];
    dp[0][0][0] = 0;
    dp[0][a[0].min(x)][b[0].min(y)] = 1;

    for i in 0..n-1 {
        for _x in 0..=x {
            for _y in 0..=y {
                if dp[i][_x][_y] == INF {
                    continue;
                }

                let nx = (_x + a[i+1]).min(x);
                let ny = (_y + b[i+1]).min(y);

                dp[i+1][_x][_y] = dp[i+1][_x][_y].min(dp[i][_x][_y]);
                dp[i+1][nx][ny] = dp[i+1][nx][ny].min(dp[i][_x][_y] + 1);
            }
        }
    }

    match dp[n-1][x][y] {
        INF => println!("-1"),
        v => println!("{}", v),
    }
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

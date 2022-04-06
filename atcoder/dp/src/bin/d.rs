fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let w: usize = scanner.cin();

    let mut dp: Vec<Vec<usize>> = vec![vec![0; w+1]; n];
    let mut ws = vec![0; n];
    let mut vs = vec![0; n];

    for i in 0..n {
        let _w: usize = scanner.cin();
        let _v: usize = scanner.cin();
        ws[i] = _w;
        vs[i] = _v;
    }

    for j in 0..=w {
        if ws[0] <= j {
            dp[0][j] = vs[0];
        }
    }

    for i in 1..n {
        for j in 0..=w {
            if j >= ws[i] {
                dp[i][j] = dp[i-1][j].max(dp[i-1][j-ws[i]] + vs[i]);
            }
            else {
                dp[i][j] = dp[i-1][j];
            }
        }
    }

    println!("{}", dp[n-1][w]);
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

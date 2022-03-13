#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut g: Vec<Vec<usize>> = vec![vec![std::usize::MAX/2; n]; n];

    for i in 0..n {
        g[i][i] = 0;
    }

    let mut es = vec![];
    for _ in 0..m {
        let a: usize = scanner.cin();
        let b: usize = scanner.cin();
        let c: usize = scanner.cin();
        g[a-1][b-1] = c;
        g[b-1][a-1] = c;
        es.push((a-1, b-1, c));
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                g[i][j] = g[i][j].min(g[i][k] + g[k][j]);
            }
        }
    }

    let mut ans: usize = 0;
    for (s, t, c) in es {
        let mut unused = 0;
        for k in 0..n {
            if s == k || t == k {
                continue
            }

            if g[s][k] + g[k][t] <= c {
                unused = 1;
            }
        }
        ans += unused;
    }

    println!("{:?}", ans);
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

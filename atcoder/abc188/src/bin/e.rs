#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let a: Vec<i64> = scanner.vec(n);

    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..m {
        let x: usize = scanner.cin::<usize>() - 1;
        let y: usize = scanner.cin::<usize>() - 1;
        graph[x].push(y);
    }

    let mut vs = (0..n).collect::<Vec<usize>>();
    vs.sort_by_key(|&x| a[x]);

    let mut used = vec![false; n];
    let mut ans: i64 = -1_000_000_000_000;

    // most reasonable prices
    let mut b = a.clone();

    for initial_v in vs {
        if used[initial_v] { continue; }

        let mut queue = VecDeque::new();
        queue.push_back(initial_v);

        while let Some(v) = queue.pop_front() {
            used[v] = true;

            for &nv in graph[v].iter() {
                ans = ans.max(a[nv] - b[v]);
                if used[nv] { continue; }

                b[nv] = a[nv].min(b[v]);
                queue.push_back(nv);
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

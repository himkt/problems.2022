const INF: usize = 1_000_000_000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut graph = vec![vec![]; n + 1];
    for _ in 0..m {
        let a: usize = scanner.cin();
        let b: usize = scanner.cin();
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut dist = vec![INF; n + 1];
    dist[1] = 0;

    let mut backptr = vec![INF; n + 1];
    backptr[1] = 0;

    let mut queue = VecDeque::new();
    queue.push_back((1, 0));

    let mut used = vec![false; n + 1];
    while let Some((u, cost)) = queue.pop_front() {
        used[u] = true;

        for &v in graph[u].iter() {
            if !used[v] && dist[v] > cost + 1 {
                dist[v] = cost + 1;
                backptr[v] = u;
                queue.push_back((v, cost + 1));
            }
        }
    }

    println!("Yes");
    for i in 2..=n {
        println!("{}", backptr[i]);
    }
}

use std::{io::Write, collections::VecDeque};
pub struct Scanner {
    buffer: std::collections::VecDeque<String>,
    buf: String,
}
#[allow(clippy::new_without_default)]
impl Scanner {
    pub fn new() -> Self {
        Scanner {
            buffer: std::collections::VecDeque::new(),
            buf: String::new(),
        }
    }
    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        if !self.buffer.is_empty() {
            return self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap();
        }
        self.buf.truncate(0);
        std::io::stdin().read_line(&mut self.buf).ok();
        self.buf
            .to_owned()
            .split_whitespace()
            .for_each(|x| self.buffer.push_back(String::from(x)));
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
    pub fn flush(&self) {
        std::io::stdout().flush().unwrap();
    }
}

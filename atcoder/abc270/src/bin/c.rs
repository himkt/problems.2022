#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let x: usize = scanner.cin::<usize>() - 1;
    let y: usize = scanner.cin::<usize>() - 1;

    let mut graph = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let u: usize = scanner.cin::<usize>() - 1;
        let v: usize = scanner.cin::<usize>() - 1;
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut backptr: Vec<usize> = (0..n).collect();
    let mut used = vec![false; n];
    let mut queue = VecDeque::new();
    queue.push_back(x);

    while let Some(u) = queue.pop_front() {
        if cfg!(debug_assertions) {
            println!("u={}", u);
        }
        used[u] = true;
        for &v in graph[u].iter() {
            if used[v] {
                continue;
            }
            if cfg!(debug_assertions) {
                println!("v={}", v);
            }
            backptr[v] = u;
            queue.push_back(v);
        }
    }

    let mut p = vec![];
    let mut cur = y;
    while cur != x {
        p.push(cur);
        cur = backptr[cur];
    }
    p.push(cur);

    for pi in p.iter().rev() {
        println!("{}", pi + 1);
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

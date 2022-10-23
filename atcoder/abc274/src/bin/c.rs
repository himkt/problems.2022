const INF: usize = 1_000_000_000_000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec::<usize>(n);

    let mut graph: Vec<Vec<usize>> = vec![vec![]; 2 * n + 2];
    for i in 0..n {
        let ai = a[i];
        let bi = 2 * (i + 1);
        let ci = 2 * (i + 1) + 1;
        graph[ai].push(bi);
        graph[ai].push(ci);
    }

    for row in graph.iter() {
        debug!("{:?}", row);
    }

    let mut dist = vec![INF; 2 * n + 2];

    let mut queue = VecDeque::new();
    queue.push_back((1, 0));

    while let Some((ci, cost)) = queue.pop_front() {
        dist[ci] = cost;

        for &ni in graph[ci].iter() {
            if cost + 1 < dist[ni] {
                queue.push_back((ni, cost + 1));
            }
        }
    }

    debug!("{:?}", dist);
    for i in 1..=(2 * n + 1) {
        println!("{}", dist[i]);
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

#[macro_export]
macro_rules! debug {
    () => {
        #[cfg(debug_assertions)]
        println!();
    };
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}

#[macro_export]
macro_rules! ndarray {
    ($x:expr;) => { $x };
    ($x:expr; $size:expr $( , $rest:expr )*) => {
        vec![ndarray!($x; $($rest),*); $size]
    };
}

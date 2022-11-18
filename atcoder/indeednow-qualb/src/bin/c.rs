#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut graph = vec![vec![]; n];
    for _ in 0..(n - 1) {
        let u: usize = scanner.cin::<usize>() - 1;
        let v: usize = scanner.cin::<usize>() - 1;
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut used = vec![false; n];
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse(0));

    let mut ans = vec![];
    while let Some(Reverse(u)) = priority_queue.pop() {
        if used[u] {
            continue;
        }
        used[u] = true;
        ans.push(u + 1);
        for &v in graph[u].iter() {
            if used[v] {
                continue;
            }
            priority_queue.push(Reverse(v));
        }
    }

    let ans = ans.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ");
    println!("{}", ans);
}

use std::{io::Write, collections::BinaryHeap, cmp::Reverse};
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

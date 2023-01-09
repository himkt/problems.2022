#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut graph = vec![vec![]; n];
    let mut degs = vec![0; n];
    for _ in 0..(n - 1) {
        let a: usize = scanner.cin::<usize>() - 1;
        let b: usize = scanner.cin::<usize>() - 1;
        graph[a].push(b);
        graph[b].push(a);
        degs[a] += 1;
        degs[b] += 1;
    }
    let mut cs: Vec<usize> = scanner.vec(n);
    cs.sort_unstable();
    cs.reverse();

    let mut vs = vec![0; n];
    let mut i = 0;

    let mut queue = VecDeque::new();
    queue.push_back(0);

    while let Some(x) = queue.pop_front() {
        vs[x] = cs[i];
        i += 1;

        for &nx in graph[x].iter() {
            if vs[nx] == 0 {
                queue.push_back(nx);
            }
        }
    }

    let m: usize = cs.iter().sum::<usize>() - cs[0];
    println!("{}", m);
    for v in vs {
        println!("{}", v);
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

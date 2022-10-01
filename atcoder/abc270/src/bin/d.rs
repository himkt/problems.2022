#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Turn { T, A }

pub struct Solver {
    a: Vec<usize>,
    k: usize,
    memo: HashMap<(usize, Turn), usize>,
}

impl Solver {
    fn new(a: Vec<usize>, k: usize) -> Self {
        Solver { a, k, memo: HashMap::new() }
    }

    fn solve(&mut self, n: usize, turn: Turn) -> usize {
        if n == 0 {
            return 0;
        }
        if self.memo.contains_key(&(n, turn)) {
            return self.memo[&(n, turn)];
        }

        let mut vs = vec![];
        for i in 0..self.k {
            if n < self.a[i] {
                continue;
            }
            let v = match &turn {
                Turn::T => self.solve(n - self.a[i], Turn::A) + self.a[i],
                Turn::A => self.solve(n - self.a[i], Turn::T),
            };
            vs.push(v);
        }

        let ret = match &turn {
            Turn::T => *vs.iter().max().unwrap(),
            Turn::A => *vs.iter().min().unwrap(),
        };

        self.memo.insert((n, turn), ret);
        ret
    }
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(k);
    let mut solver = Solver::new(a, k);
    let ans = solver.solve(n, Turn::T);
    println!("{}", ans);
}

use std::{io::Write, collections::HashMap};
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

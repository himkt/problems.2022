fn distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
    let v1 = p1.0.max(p2.0) - p1.0.min(p2.0);
    let v2 = p1.1.max(p2.1) - p1.1.min(p2.1);
    v1 + v2
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();
    let d: usize = scanner.cin();

    let mut id2position = HashMap::new();
    for i in 0..h {
        for j in 0..w {
            let aij = scanner.cin::<usize>() - 1;
            id2position.insert(aij, (i, j));
        }
    }

    let q: usize = scanner.cin();
    let mut ls = vec![0; q];
    let mut rs = vec![0; q];
    for i in 0..q {
        let l = scanner.cin::<usize>() - 1;
        let r = scanner.cin::<usize>() - 1;
        ls[i] = l;
        rs[i] = r;
    }

    let n: usize = h * w;
    let mut dist = ndarray!(0; n);

    for u in 0..n {
        if dist[u] != 0 {
            continue;
        }
        let mut v = u + d;
        while v < n {
            let p1 = id2position[&(v - d)];
            let p2 = id2position[&v];
            dist[v] = dist[v - d] + distance(p1, p2);
            v += d;
        }
    }

    debug!("{:?}", dist);
    for i in 0..q {
        let mut ans = dist[rs[i]];
        if ls[i] >= d {
            ans -= dist[ls[i]];
        }
        println!("{}", ans);
    }
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

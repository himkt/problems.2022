fn dist(l: usize, r: usize, points: &HashMap<usize, (i64, i64)>) -> i64 {
    let lp = points[&l];
    let rp = points[&r];
    (lp.0 - rp.0).abs() + (lp.1 - rp.1).abs()
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();
    let d: usize = scanner.cin();

    let mut points = HashMap::new();
    for i in 0..h {
        for j in 0..w {
            let aij: usize = scanner.cin::<usize>() - 1;
            points.insert(aij, (i as i64, j as i64));
        }
    }

    debug!("{:?}", points);
    let mut dp = vec![0; h * w];
    for s in 0..d {
        let mut t = s + d;
        while t < h * w {
            dp[t] += dp[t - d] + dist(t - d, t, &points);
            t += d;
        }
    }

    debug!("{:?}", dp);
    let q: usize = scanner.cin();
    for _ in 0..q {
        let l: usize = scanner.cin::<usize>() - 1;
        let r: usize = scanner.cin::<usize>() - 1;
        println!("{}", dp[r] - dp[l])
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

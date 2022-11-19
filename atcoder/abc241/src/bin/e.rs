#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);
    let mut hist = HashMap::new();

    let mut cursor = 0;
    let mut num_iters = 0;
    let mut tot = vec![0];
    loop {
        let u = cursor % n;
        if hist.contains_key(&u) {
            debug!("hist={} current={}", hist[&u], num_iters + 1);
            let p: usize = num_iters + 1 - hist[&u];
            let m: usize = tot.len();
            for i in 1..m {
                tot[i] += tot[i - 1];
            }
            let mut ans = tot[hist[&u] - 1];
            let mut ck = k - hist[&u] + 1;
            debug!("{:?}", tot);
            debug!("ck={}, p={}", ck, p);
            debug!("{}", ans);
            ans += (ck / p) * (tot[m - 1] - tot[hist[&u] - 1]);
            ck -= (ck / p) * p;
            debug!("== after ==");
            debug!("ck={}, p={}", ck, p);
            if ck > 0 {
                ans += tot[hist[&u] + ck - 1] - tot[hist[&u] - 1];
            }
            println!("{}", ans);
            break;
        }
        if num_iters == k {
            let ans: usize = tot.iter().sum();
            println!("{}", ans);
            break;
        }
        num_iters += 1;
        debug!("write hist {} {}", u, num_iters);
        hist.insert(u, num_iters);
        cursor += a[u];
        tot.push(a[u]);
        debug!("cursor={}", cursor);
    }

    debug!("{:?}", hist);
    debug!("{:?}", tot);
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

const MAX: i64 = 10_000;
const MIN: i64 = - 10_000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let x: i64 = scanner.cin();
    let y: i64 = scanner.cin();
    let a: Vec<i64> = scanner.vec(n);

    let mut b = vec![];
    let mut c = vec![];
    for i in 0..n {
        if i % 2 == 0 {
            b.push(a[i]);
        }
        else {
            c.push(a[i]);
        }
    }

    let ni = b.len();
    let nj = c.len();

    let mut dpb = vec![HashMap::new(); ni + 1];
    dpb[1].insert(b[0], true);

    for i in 2..=ni {
        for j in MIN..=MAX {
            if dpb[i - 1].contains_key(&j) {
                dpb[i].insert(j + b[i - 1], true);
                dpb[i].insert(j - b[i - 1], true);
            }
        }
    }

    let mut dpc = vec![HashMap::new(); nj + 1];
    dpc[0].insert(0, true);
    for i in 1..=nj {
        for j in MIN..=MAX {
            if dpc[i - 1].contains_key(&j) {
                dpc[i].insert(j + c[i - 1], true);
                dpc[i].insert(j - c[i - 1], true);
            }
        }
    }

    debug!("dpb={:?}", dpb[ni]);
    debug!("dpc={:?}", dpc[nj]);
    if dpb[ni].contains_key(&x) && dpc[nj].contains_key(&y) {
        println!("Yes");
    }
    else {
        println!("No");
    }
}

use std::{io::Write, collections::{HashMap}};
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

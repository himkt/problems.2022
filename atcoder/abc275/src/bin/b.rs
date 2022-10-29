const DIV: usize = 998244353;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let a: usize = scanner.cin::<usize>() % DIV;
    let b: usize = scanner.cin::<usize>() % DIV;
    let c: usize = scanner.cin::<usize>() % DIV;
    let d: usize = scanner.cin::<usize>() % DIV;
    let e: usize = scanner.cin::<usize>() % DIV;
    let f: usize = scanner.cin::<usize>() % DIV;
    let v1 = (a * b) % DIV;
    let v2 = (v1 * c) % DIV;
    let r1 = (d * e) % DIV;
    let r2 = (r1 * f) % DIV;
    let ans = (DIV + v2 - r2) % DIV;
    println!("{}", ans);
}

use std::io::Write;
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

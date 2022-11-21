#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let d = n + 1;
    let a: Vec<usize> = scanner.vec(d);
    let mut b: Vec<usize> = vec![0; d];
    b[0] = 1;

    let mut tot: Vec<usize> = vec![0; d];
    tot[d - 1] = a[d - 1];
    for c in (0..(d - 1)).rev() {
        tot[c] += tot[c + 1] + a[c];
    }

    if n == 0 {
        if a[0] == 1 {
            println!("1");
        }
        else {
            println!("-1");
        }
        return;
    }

    if n == 1 && a[0] > 0 && tot[d - 1] > 0 {
        println!("-1");
        return;
    }

    debug!("{:?}", a);
    debug!("{:?}", tot);
    let mut ans = 1;
    for c in 1..d {
        if b[c - 1] == 0 {
            println!("-1");
            return;
        }
        let actual = (b[c - 1] * 2).min(tot[c]);
        debug!("d={}, actual={}, b={:?}, tot={:?}", c, actual, b, tot);
        ans += actual;
        b[c] = actual;
        if b[c] < a[c] {
            println!("-1");
            return;
        }
        b[c] -= a[c];
        debug!("b={:?}", b);
    }

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

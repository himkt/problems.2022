fn solve(a: usize, b: usize) -> usize {
    let mut r = a;
    let mut l = b;

    let f = |x: usize| {
        let q = x;

        let mut ret = 1_000_000_000;
        debug!("q={}, a={}, b={}", q, a, b);

        if q >= a {
            ret = ret.min(q - a);
        }
        if 2 * q >= a {
            ret = ret.min(2 * q - a);
        }
        if 3 * q >= a {
            ret = ret.min(3 * q - a);
        }
        debug!("ret={}", ret);
        (x - b) + ret
    };

    for _ in 0..500 {
        let c1 = (2 * l + r) / 3;
        let c2 = (l + 2 * r) / 3;
        debug!("l={}, r={}, f1={}, f2={}", l, r, f(c1), f(c2));

        if f(c1) > f(c2) {
            l = c1;
        }
        else {
            r = c2;
        }
    }

    let mut ans = 1_000_000_000;
    for i in l..=r {
        debug!("f(i={})={}", i, f(i));
        ans = ans.min(f(i));
    }
    debug!("{}", ans);
    ans

}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let t: usize = scanner.cin();

    for _ in 0..t {
        let mut a: usize = scanner.cin();
        let mut b: usize = scanner.cin();
        if a < b {
            swap(&mut a, &mut b);
        }
        println!("{}", solve(a, b));
    }
}

use std::{io::Write, mem::swap};
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

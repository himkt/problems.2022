const INF: i64 = 1_000_000_000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m = n + 2;
    let s: Vec<i64> = scanner.vec(n);
    let mut a = vec![0; m];

    a[0] = s[0];
    for i in 3..m {
        a[i] = s[i - 2] - s[i - 3] + a[i - 3];
    }

    let mut diff = vec![INF; 3];
    for i in 0..m {
        diff[i % 3] = diff[i % 3].min(a[i]);
    }
    let diff0 = diff.clone();

    for i in 0..3 {
        for j in 0..3 {
            if i == j {
                continue;
            }
            if diff[i] > 0 && diff[j] < 0 {
                let pad = diff[j].abs().min(diff[i]);
                diff[i] -= pad;
                diff[j] += pad;
            }
        }
    }

    for i in 0..m {
        a[i] += diff[i % 3] - diff0[i % 3];
        if a[i] < 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
    for ai in a {
        println!("{:?}", ai);
    }
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

const INF: i64 = 1_000_000_000;

fn d(u: usize, v: usize, points: &[(i64, i64, i64)]) -> i64 {
    let (a, b, c) = points[u];
    let (p, q, r) = points[v];
    (p - a).abs() + (q - b).abs() + 0.max(r - c)
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let points: Vec<(i64, i64, i64)> = (0..n).map(|_| {
        let x: i64 = scanner.cin();
        let y: i64 = scanner.cin();
        let z: i64 = scanner.cin();
        (x, y, z)
    }).collect();

    let s: usize = 0;
    let mut dp = vec![vec![INF; 1 << n]; n];
    for u in 0..n {
        dp[u][1 << u] = d(s, u, &points);
    }
    debug!("[init] {:?}", dp);

    for bit in 0..(1 << n) {
        for u in 0..n {
            if (bit >> u) & 1 == 0 {
                continue;
            }

            for v in 0..n {
                if (bit >> v) & 1 == 1 {
                    continue;
                }
                let tmp = dp[u][bit] + d(u, v, &points);
                let nbit = bit | (1 << v);
                if tmp < dp[v][nbit] {
                    dp[v][nbit] = tmp;
                }
            }
        }
    }

    debug!("[after] {:?}", dp);
    println!("{}", dp[s][(1 << n) - 1]);
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

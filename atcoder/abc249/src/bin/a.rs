#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let a: usize = scanner.cin();
    let b: usize = scanner.cin();
    let c: usize = scanner.cin();
    let d: usize = scanner.cin();
    let e: usize = scanner.cin();
    let f: usize = scanner.cin();
    let x: usize = scanner.cin();

    let mut d1 = 0;
    let p1 = x / (a + c);

    d1 += a * b * p1;
    if x > p1 * (a + c) {
        let q = x - p1 * (a + c);
        d1 += b * q.min(a);
    }

    let mut d2 = 0;
    let p2 = x / (d + f);
    d2 += d * e * p2;
    if x > p2 * (d + f) {
        let q = x - p2 * (d + f);
        d2 += e * q.min(d);
    }

    let ans = match d1.cmp(&d2) {
        Ordering::Equal => "Draw",
        Ordering::Greater => "Takahashi",
        Ordering::Less => "Aoki",
    };

    println!("{}", ans);
}


use std::cmp::Ordering;
use std::collections::VecDeque;
use std::io::{self, Write};
use std::str::FromStr;

#[allow(dead_code)]
struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
#[allow(dead_code)]
impl Scanner {
    fn new() -> Self {
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
    }

    fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

#[allow(dead_code)]
fn flush() {
    std::io::stdout().flush().unwrap();
}

#[macro_export]
macro_rules! trace {
    ($x:expr) => {
        #[cfg(debug_assertions)]
        eprintln!(">>> {} = {:?}", stringify!($x), $x)
    };
    ($($xs:expr),*) => { trace!(($($xs),*)) }
}

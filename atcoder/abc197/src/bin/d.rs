#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let x0: f64 = scanner.cin();
    let y0: f64 = scanner.cin();
    let xh: f64 = scanner.cin();
    let yh: f64 = scanner.cin();

    let cx = (x0 + xh) / 2.0;
    let cy = (y0 + yh) / 2.0;

    let rad = 2.0 * PI / (n as f64);
    let x = rad.cos() * (x0 - cx) - rad.sin() * (y0 - cy) + cx;
    let y = rad.sin() * (x0 - cx) + rad.cos() * (y0 - cy) + cy;

    println!("{} {}", x, y);
}


use std::collections::VecDeque;
use std::f64::consts::PI;
use std::io::{self, Write};
use std::str::FromStr;

pub struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
impl Scanner {
    fn new() -> Self {
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
    }

    pub fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

pub fn flush() {
    std::io::stdout().flush().unwrap();
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut x = vec![vec![0; n]; n];

    let mut v = 1;
    for i in 0..n {
        if i % 2 == 1 {
            continue;
        }
        for j in 0..n {
            x[i][j] = v;
            v += 1;
        }
    }

    for i in (0..n).rev() {
        if i % 2 == 0 {
            continue;
        }
        for j in 0..n {
            x[i][j] = v;
            v += 1;
        }
    }

    for i in 0..n {
        let xi = x[i]
            .iter()
            .map(|&x_ij| x_ij.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", xi);
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

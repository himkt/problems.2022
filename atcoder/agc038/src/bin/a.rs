#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();
    let a: usize = scanner.cin();
    let b: usize = scanner.cin();
    let mut board = vec![vec!["0"; w]; h];

    for i in 0..b {
        for j in 0..a {
            board[i][j] = "1";
        }
    }
    for i in b..h {
        for j in a..w {
            board[i][j] = "1";
        }
    }

    for si in board {
        let si: String = si.into_iter().collect();
        println!("{}", si);
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

#[derive(Debug)]
pub enum Color {
    Black,
    White,
}

fn get_color(i: usize, j: usize) -> Color {
    let dx = 8.max(i) - 8.min(i);
    let dy = 8.max(j) - 8.min(j);
    let dist = dx.max(dy);

    if dist % 2 == 0 {
        Color::White
    }
    else {
        Color::Black
    }
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let r: usize = scanner.cin();
    let c: usize = scanner.cin();

    match get_color(r, c) {
        Color::Black => println!("black"),
        Color::White => println!("white"),
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

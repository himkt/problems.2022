#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();

    let mut board: Vec<Vec<char>> = vec![vec!['x'; w]; h];
    for i in 0..h {
        let cs: Vec<char> = scanner.cin::<String>().chars().collect();
        board[i] = cs;
    }

    let mut points = vec![];
    for i in 0..h {
        for j in 0..w {
            if board[i][j] == 'o' {
                points.push((i as i64, j as i64));
            }
        }
    }

    let p1 = points[0];
    let p2 = points[1];
    let ans = (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
    println!("{}", ans);
}


use std::collections::VecDeque;
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

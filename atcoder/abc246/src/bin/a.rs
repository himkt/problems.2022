fn main() {
    let mut scanner = Scanner::new();
    let x1: i64 = scanner.cin();
    let y1: i64 = scanner.cin();
    let x2: i64 = scanner.cin();
    let y2: i64 = scanner.cin();
    let x3: i64 = scanner.cin();
    let y3: i64 = scanner.cin();

    for x4 in [x1, x2, x3] {
        for y4 in [y1, y2, y3] {
            if x1 == x4 && y1 == y4 {
                continue;
            }
            if x2 == x4 && y2 == y4 {
                continue;
            }
            if x3 == x4 && y3 == y4 {
                continue;
            }
            println!("{} {}", x4, y4);
            return;
        }
    }
}


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
        Self {
            stdin: io::stdin(),
            buffer: VecDeque::new(),
        }
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

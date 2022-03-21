fn main() {
    let mut scanner = Scanner::new();
    let _: usize = scanner.cin();
    let s: String = scanner.cin();

    let mut state = "+x";

    let mut x: i64 = 0;
    let mut y: i64 = 0;

    for c in s.chars() {
        if c == 'R' {
            state = match state {
                "+x" => "-y",
                "-y" => "-x",
                "-x" => "+y",
                "+y" => "+x",
                _ => panic!(),
            }
        }
        else {
            if state == "+x" {
                x += 1;
            }
            else if state == "-x" {
                x -= 1;
            }
            else if state == "+y" {
                y += 1;
            }
            else if state == "-y" {
                y -= 1;
            }
        }
    }

    println!("{} {}", x, y);
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

    fn usize1(&mut self) -> usize {
        self.cin::<usize>() - 1
    }

    fn chars(&mut self) -> Vec<char> {
        self.cin::<String>().chars().collect()
    }

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

#[allow(dead_code)]
fn flush() {
    std::io::stdout().flush().unwrap();
}

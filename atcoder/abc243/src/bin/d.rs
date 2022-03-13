fn main() {
    let mut scanner = Scanner::new();
    let _: usize = scanner.cin();
    let mut x: u128 = scanner.cin();
    let sraw: String = scanner.cin();
    let s: Vec<char> = sraw.chars().collect();

    let mut sminimul: VecDeque<char> = VecDeque::new();
    sminimul.push_back('x');

    for c in s {
        let back = *sminimul.back().unwrap();
        if (back == 'R' || back == 'L') && c == 'U' {
            sminimul.pop_back();
        }
        else {
            sminimul.push_back(c);
        }
    }

    for c in sminimul {
        if c == 'U' {
            x /= 2;
        }
        if c == 'L' {
            x *= 2;
        }
        if c == 'R' {
            x *= 2;
            x += 1;
        }
    }

    println!("{}", x);
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

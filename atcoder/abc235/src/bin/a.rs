fn main() {
    let mut scanner = Scanner::new();
    let s: String = scanner.cin();
    let s: Vec<char> = s.chars().collect();

    let a = s[0].to_digit(10).unwrap();
    let b = s[1].to_digit(10).unwrap();
    let c = s[2].to_digit(10).unwrap();

    let s1 = 100*a + 10*b + c;
    let s2 = 100*b + 10*c + a;
    let s3 = 100*c + 10*a + b;
    println!("{}", s1 + s2 + s3);
}


use std::io::{self, Write};
use std::str::FromStr;
use std::collections::VecDeque;

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

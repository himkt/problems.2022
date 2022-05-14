#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let _: usize = scanner.cin();

    let mut a = vec![];
    for i in 1..=99 {
        a.push(i);
        a.push(100 * i);
        a.push(10000 * i);
    }

    let k = a.len();
    let ans: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    let ans: String = ans.join(" ");
    println!("{}", k);
    println!("{}", ans);
}


use std::collections::{VecDeque};
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

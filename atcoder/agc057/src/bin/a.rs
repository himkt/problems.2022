#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let t: usize = scanner.cin();

    for _ in 0..t {
        let l: usize = scanner.cin();
        let r: usize = scanner.cin();

        let dl = (l as f64).log10() as u32;
        let dr = (r as f64).log10() as u32;

        if dl == dr {
            println!("{}", r - l + 1);
            continue;
        }

        let v = 10usize.pow(dr);
        if 2 * v - 1 <= r {
            println!("{}", r - v + 1);
            continue;
        }

        let p = ((r % v) + 1).max((r / 10) + 1).max(l);
        println!("{}", r - p + 1);
    }
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

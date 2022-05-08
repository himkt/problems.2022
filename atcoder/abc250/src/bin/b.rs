#[allow(clippy::needless_range_loop)]
#[allow(clippy::collapsible_else_if)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: usize = scanner.cin();
    let b: usize = scanner.cin();

    let na = n * a;
    let nb = n * b;

    let mut ans = vec![];

    for i in 0..na {
        let mut cs = vec![];
        for j in 0..nb {

            let pa = (i / a) % 2;
            let pb = (j / b) % 2;

            if pa == 0 {
                if pb == 0 {
                    cs.push('.');
                }
                else {
                    cs.push('#');
                }
            }
            else {
                if pb == 0 {
                    cs.push('#');
                }
                else {
                    cs.push('.');
                }
            }
        }

        let si: String = cs.iter().collect();
        ans.push(si);
    }

    for si in ans {
        println!("{}", si);
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

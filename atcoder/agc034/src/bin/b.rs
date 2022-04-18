#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let s: String = scanner.cin();
    let cs: Vec<char> = s.chars().collect();
    let n: usize = cs.len();

    let mut prev = cs[0];
    let mut acnt: usize = 0;

    if cs[0] == 'A' {
        acnt += 1;
    }

    let mut ans = 0;

    for i in 1..n {
        match cs[i] {
            'A' => {
                if prev == 'A' { acnt += 1; }
                else { acnt = 1; }
            },
            'B' => {
                if prev == 'A' { }
                else { acnt = 0; }
            },
            'C' => {
                if prev == 'B' {
                    ans += acnt;
                    prev = 'A';
                    continue;
                }
                else { acnt = 0; }
            },
            _ => panic!(),
        }

        prev = cs[i];
    }

    println!("{}", ans);
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
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
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

#[macro_export]
macro_rules! trace {
    ($x:expr) => {
        #[cfg(debug_assertions)]
        eprintln!(">>> {} = {:?}", stringify!($x), $x)
    };
    ($($xs:expr),*) => { trace!(($($xs),*)) }
}

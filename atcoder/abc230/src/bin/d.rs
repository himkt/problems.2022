#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let d: usize = scanner.cin();

    let mut lrs: Vec<(usize, usize)> = (0..n)
        .map(|_| {
            let l: usize = scanner.cin();
            let r: usize = scanner.cin();
            (l, r)
        })
        .collect();

    lrs.sort_by_key(|&(_, r)| r);

    let mut ans: usize = 1;
    let mut prev = lrs[0].1;

    for i in 1..n {
        if lrs[i].0 < prev + d {
            continue;
        }
        ans += 1;
        prev = lrs[i].1;
    }

    println!("{}", ans);
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

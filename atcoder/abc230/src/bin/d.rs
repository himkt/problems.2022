fn main() {
    let mut scanner = Scanner::new();

    let n: usize = scanner.cin();
    let d: i64 = scanner.cin();

    let mut xys: Vec<(i64, i64)> = vec![];
    for _ in 0..n {
        let l: i64 = scanner.cin();
        let r: i64 = scanner.cin();
        xys.push((l-1, r-1));
    }
    xys.sort_by(|&(_, ay), &(_, by)| ay.cmp(&by));

    let mut ans = 0;
    let mut c: i64 = i64::MIN;

    for &(l, r) in &xys {
        if c + d > l {
            continue;
        }
        c = r;
        ans += 1;
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

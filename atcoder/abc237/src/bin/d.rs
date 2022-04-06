fn main() {
    let mut scanner = Scanner::new();
    let mut n: i64 = scanner.cin();
    let s: String = scanner.cin();
    let s: Vec<char> = s.chars().collect();

    let mut a: Vec<i64> = vec![-1; 1_000_001];
    let c: usize = 500_000;

    let mut l: usize = 0;
    let mut r: usize = 0;

    a[c] = n;
    n -= 1;

    for si in s.into_iter().rev() {
        if si == 'R' {
            l += 1;
            a[c-l] = n;
        }
        else {
            r += 1;
            a[c+r] = n;
        }
        n -= 1;
    }

    let mut ans = vec![];
    for ai in a {
        if ai >= 0 {
            ans.push(ai.to_string());
        }
    }
    println!("{}", ans.join(" "));
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

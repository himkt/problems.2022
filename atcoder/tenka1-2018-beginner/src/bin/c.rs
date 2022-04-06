fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut a: Vec<i64> = scanner.vec(n);
    a.sort_unstable();

    let mut deque1: VecDeque<i64> = VecDeque::from(a);
    let mut deque2: VecDeque<i64> = deque1.clone();
    let mut b1: Vec<i64> = vec![0; n];
    let mut b2: Vec<i64> = vec![0; n];

    let c = n / 2;
    b1[c] = deque1.pop_front().unwrap();

    for i in 1..=n {
        if i % 2 == 1 {
            if i <= c {
                b1[c-i] = deque1.pop_back().unwrap();
            }
            if c + i < n {
                b1[c+i] = deque1.pop_back().unwrap();
            }
        }
        else {
            if i <= c {
                b1[c-i] = deque1.pop_front().unwrap();
            }
            if c + i < n {
                b1[c+i] = deque1.pop_front().unwrap();
            }
        }
    }

    let mut ans1: i64 = 0;
    for i in 0..n-1 {
        ans1 += (b1[i] - b1[i+1]).abs();
    }

    b2[c] = deque2.pop_back().unwrap();

    for i in 1..=n {
        if i % 2 == 1 {
            if i <= c {
                b2[c-i] = deque2.pop_front().unwrap();
            }
            if c + i < n {
                b2[c+i] = deque2.pop_front().unwrap();
            }
        }
        else {
            if i <= c {
                b2[c-i] = deque2.pop_back().unwrap();
            }
            if c + i < n {
                b2[c+i] = deque2.pop_back().unwrap();
            }
        }
    }

    let mut ans2: i64 = 0;
    for i in 0..n-1 {
        ans2 += (b2[i] - b2[i+1]).abs();
    }

    let ans = ans1.max(ans2);

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

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

#[allow(dead_code)]
fn flush() {
    std::io::stdout().flush().unwrap();
}

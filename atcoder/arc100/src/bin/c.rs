const MAX: f64 = 2e14;


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<f64> = scanner.vec(n);
    let mut c: Vec<f64> = vec![0.0; n];

    for i in 0..n {
        c[i] = a[i] - (i as f64 + 1.0);
    }

    let f = |b: f64| {
        let mut v = 0.0;
        for i in 0..n {
            v += (c[i] - b).abs();
        }
        v
    };

    let mut l = -MAX;
    let mut r = MAX;

    for _ in 0..500 {
        let c1 = (2.0*l + r) / 3.0;
        let c2 = (l + 2.0*r) / 3.0;

        if f(c1) > f(c2) {
            l = c1;
        }
        else {
            r = c2;
        }
    }

    let mut ans = MAX;
    let left = l as i64 - 5;
    let right = r as i64 + 5;
    for i in left..=right {
        ans = ans.min(f(i as f64));
    }

    // println!("{}, {} {}", l, r, ans);
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

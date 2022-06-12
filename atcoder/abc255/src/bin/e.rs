#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let s: Vec<i128> = scanner.vec(n - 1);
    let x: Vec<i128> = scanner.vec(m);

    let mut b = vec![0; n];
    for i in 1..n {
        b[i] = s[i - 1] - b[i - 1];
    }

    let mut cnt: HashMap<i128, usize> = HashMap::new();
    for i in 0..n {
        for j in 0..m {
            let sign = if i % 2 == 0 { 1 } else { -1 };

            // a[i] が lucky number (x[j]) である場合の初項
            let a_0 = sign * (x[j] - b[i]);
            *cnt.entry(a_0).or_insert(0) += 1;
        }
    }

    let mut ans = 0;
    for (_, v) in cnt {
        ans = ans.max(v);
    }

    println!("{:?}", ans);
}


use std::collections::{VecDeque, HashMap};
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

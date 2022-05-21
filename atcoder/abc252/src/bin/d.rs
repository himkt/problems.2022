#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut cnt = HashMap::new();
    for &ai in a.iter() {
        *cnt.entry(ai).or_insert(0) += 1;
    }

    let mut b: Vec<usize> = cnt.keys().cloned().collect();
    b.sort_unstable();

    let m = b.len();
    let c: Vec<usize> = (0..m).map(|x| cnt[&b[x]]).collect();

    let mut cum = c.clone();
    for i in 1..m {
        cum[i] += cum[i - 1];
    }

    let mut ans = 0;
    for i in 1..m-1 {
        let v = cum[i - 1] * (n - cum[i]);
        ans += c[i] * v;
    }

    println!("{}", ans);
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

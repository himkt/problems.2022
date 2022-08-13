const INF: i64 = 1_000_000_000_000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();
    let x: Vec<i64> = scanner.vec(n);

    let mut ans = INF;

    let mut l;
    let mut r;
    for i in 0..=(n - k) {
        l = x[i];
        r = x[i + k - 1];

        if l == 0 || r == 0 {
            ans = ans.min(r - l);
        }
        else if l * r < 0 {
            let min = r.abs().min(l.abs());
            let max = r.abs().max(l.abs());
            ans = ans.min(max + 2 * min);
        }
        else {
            if l < 0 {
                ans = ans.min(l.abs());
            }
            else {
                ans = ans.min(r.abs());
            }
        }
    }
    println!("{}", ans);
}

use std::io::Write;
pub struct Scanner {
    buffer: std::collections::VecDeque<String>,
    buf: String,
}
#[allow(clippy::new_without_default)]
impl Scanner {
    pub fn new() -> Self {
        Scanner {
            buffer: std::collections::VecDeque::new(),
            buf: String::new(),
        }
    }
    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        if !self.buffer.is_empty() {
            return self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap();
        }
        self.buf.truncate(0);
        std::io::stdin().read_line(&mut self.buf).ok();
        self.buf
            .to_owned()
            .split_whitespace()
            .for_each(|x| self.buffer.push_back(String::from(x)));
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
    pub fn flush(&self) {
        std::io::stdout().flush().unwrap();
    }
}

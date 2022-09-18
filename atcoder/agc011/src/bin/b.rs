#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut a: Vec<(usize, usize)> = (0..n)
        .map(|i| {
            let ai: usize = scanner.cin();
            (i, ai)
        })
        .collect();
    a.sort_unstable_by_key(|&(_, ai)| ai);

    let mut dp = vec![0; n];
    dp[0] = 1;

    let mut tot = a[0].1;
    for i in 1..n {
        if 2 * tot >= a[i].1 {
            dp[i] = dp[i - 1] + 1;
        }
        else {
            dp[i] = 1;
        }
        tot += a[i].1;
    }

    println!("{}", dp[n - 1]);
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

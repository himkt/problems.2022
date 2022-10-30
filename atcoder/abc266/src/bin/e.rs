#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut dp = vec![0.0; n + 1];
    dp[1] = 3.5;

    for i in 2..=n {
        // never reaching at 6
        let x: usize = dp[i - 1] as usize;

        let lower = x;
        let upper = 6 - x;

        let sum_lower = (x * (1 + x)) / 2;
        let sum_upper = 21 - sum_lower;

        let p_lower = lower as f64 / 6.0;
        let p_upper = 1.0 / upper as f64;

        let e_lower = p_lower * dp[i - 1];
        let e_upper = (1.0 - p_lower) * (sum_upper as f64 * p_upper);
        dp[i] = e_lower + e_upper;
    }

    println!("{}", dp[n]);
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

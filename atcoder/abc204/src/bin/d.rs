const T: usize = 100_001;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let t: Vec<usize> = scanner.vec(n);

    let mut dp = vec![vec![0; T]; n + 1];
    dp[0][0] = 1;

    for i in 1..=n {
        for j in 0..T {
            if j + t[i-1] < T {
                dp[i][j + t[i-1]] += dp[i - 1][j];
                dp[i][j + t[i-1]] %= T;
            }
            dp[i][j] += dp[i - 1][j];
            dp[i][j] %= T;
        }
    }

    let mut ans = 1_000_000;
    let s: usize = t.iter().sum();

    for si in 1..T {
        if dp[n][si] > 0 {
            ans = ans.min(si.max(s - si));
        }
    }

    println!("{}", ans);
}

use std::io::Write; pub fn flush() { std::io::stdout().flush().unwrap(); }
pub struct Scanner { buffer: std::collections::VecDeque<String>, buf: String }
impl Scanner {
    pub fn new() -> Self { Scanner { buffer: std::collections::VecDeque::new(), buf: String::new() } }
    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        if !self.buffer.is_empty() { return self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap(); }
        self.buf.truncate(0); std::io::stdin().read_line(&mut self.buf).ok();
        self.buf.to_owned().split_whitespace().for_each(|x| self.buffer.push_back(String::from(x)));
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> { (0..n).map(|_| self.cin()).collect() }
}

const DIV: usize = 998244353;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut a: Vec<usize> = scanner.vec(n);
    a.sort_unstable();

    let mut ans = 0;
    for i in 0..n {
        ans += a[i] * a[i];
        ans %= DIV;
    }

    if n > 1 {
        let mut sum = 0;
        let mut r = a[n - 1];

        // sum_{i=1}^{N-1} f(i)
        for i in (0..(n - 1)).rev() {
            if i < n - 2 {
                r = a[i + 1] + 2 * r;
                r %= DIV;
            }
            sum += a[i] * r;
            sum %= DIV;
        }
        ans += sum;
        ans %= DIV;
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

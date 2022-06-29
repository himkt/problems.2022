pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut fgcd = vec![a[0]; n];
    let mut bgcd = vec![a[n - 1]; n];

    for i in 1..n {
        fgcd[i] = gcd(fgcd[i - 1], a[i]);
    }

    for i in (0..(n - 1)).rev() {
        bgcd[i] = gcd(bgcd[i + 1], a[i]);
    }

    let mut ans = 1;
    ans = ans.max(fgcd[n - 2]);
    ans = ans.max(bgcd[1]);
    for i in 1..(n - 1) {
        ans = ans.max(gcd(fgcd[i - 1], bgcd[i + 1]));
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

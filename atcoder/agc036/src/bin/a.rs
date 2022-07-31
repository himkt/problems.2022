#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let s: usize = scanner.cin();
    let x1 = 1_000_000_000;
    let y2 = (s + (x1 - 1)) / x1;
    let x2 = 1;
    let y1 = x1 * y2 - s;
    let x3 = 0;
    let y3 = 0;
    println!("{} {} {} {} {} {}", x1, y1, x2, y2, x3, y3);
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

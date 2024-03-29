#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let a: f64 = scanner.cin();
    let b: f64 = scanner.cin();
    let d: f64 = scanner.cin();

    let rad = std::f64::consts::PI * (d / 180.0);
    let x = rad.cos() * a - rad.sin() * b;
    let y = rad.sin() * a + rad.cos() * b;
    println!("{} {}", x, y);
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

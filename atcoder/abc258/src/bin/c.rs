#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let q: usize = scanner.cin();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();

    let mut cursor = 0;
    for _ in 0..q {
        let t: usize = scanner.cin();
        let x: usize = scanner.cin();

        if t == 1 {
            if cursor >= x {
                cursor -= x;
            }
            else {
                cursor = n - (x - cursor);
            }
        }
        else {
            if cursor + x - 1 < n {
                println!("{}", s[cursor + x - 1]);
            }
            else {
                let p = (cursor + x - 1) - n;
                println!("{}", s[p]);
            }
        }
    }
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

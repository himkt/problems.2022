#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let cs: Vec<char> = scanner.cin::<String>().chars().collect();
    let mut rcs: Vec<char> = cs.clone();
    rcs.reverse();

    let s: String = cs.iter().collect();
    let rs: String = rcs.iter().collect();

    if s == rs {
        println!("Yes");
    }
    else {
        println!("No");
    }
}
use std::io::Write;
pub fn flush() { std::io::stdout().flush().unwrap(); }
pub struct Scanner { stdin: std::io::Stdin, buffer: std::collections::VecDeque<String> }
impl Scanner {
    fn new() -> Self {
        Self { stdin: std::io::stdin(), buffer: std::collections::VecDeque::new() }
    }
    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            line.split_whitespace().for_each(|x| self.buffer.push_back(String::from(x)));
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

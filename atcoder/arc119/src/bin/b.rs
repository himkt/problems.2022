#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let _: usize = scanner.cin();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();
    let t: Vec<char> = scanner.cin::<String>().chars().collect();

    let s_0s: Vec<usize> = s.iter().enumerate().filter(|&(_, c)| c == &'0').map(|(i, _)| i).collect();
    let t_0s: Vec<usize> = t.iter().enumerate().filter(|&(_, c)| c == &'0').map(|(i, _)| i).collect();

    if s_0s.len() != t_0s.len() {
        println!("-1");
        return;
    }

    let n_0s = s_0s.len();
    let mut v_0s = 0;
    for i in 0..n_0s {
        if s_0s[i] != t_0s[i] {
            v_0s += 1;
        }
    }

    let ans = v_0s;
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

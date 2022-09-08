fn estimate_cost(s: &[char], c: char) -> usize {
    let mut costs: Vec<usize> = vec![];
    let mut cur = 0;

    for &ci in s {
        if ci == c {
            costs.push(cur);
            cur = 0;
        }
        else {
            cur += 1;
        }
    }

    if cur > 0 {
        costs.push(cur);
    }

    costs.iter().max().unwrap().clone()
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();

    let mut ans = s.len();
    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        ans = ans.min(estimate_cost(&s, c));
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

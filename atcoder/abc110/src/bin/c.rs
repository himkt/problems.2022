#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();
    let t: Vec<char> = scanner.cin::<String>().chars().collect();

    let mut target_chars_by_s: HashMap<char, HashSet<char>> = HashMap::new();
    let mut target_chars_by_t: HashMap<char, HashSet<char>> = HashMap::new();

    for i in 0..s.len() {
        target_chars_by_s.entry(s[i]).or_insert(HashSet::new()).insert(t[i]);
        target_chars_by_t.entry(t[i]).or_insert(HashSet::new()).insert(s[i]);
    }

    for (_, set) in target_chars_by_s {
        if set.len() > 1 {
            println!("No");
            return;
        }
    }
    for (_, set) in target_chars_by_t {
        if set.len() > 1 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

use std::{io::Write, collections::{HashMap, HashSet}};
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

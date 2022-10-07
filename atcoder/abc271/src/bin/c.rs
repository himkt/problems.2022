#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut unnecessary_books = vec![];
    let mut necessary_books = BTreeSet::new();
    for &ai in a.iter() {
        if necessary_books.contains(&ai) {
            unnecessary_books.push(ai);
        }
        else {
            necessary_books.insert(ai);
        }
    }

    let mut ans = 0;
    for target in 1..=(3 * 100000 + 1000) {
        if necessary_books.contains(&target) {
            necessary_books.remove(&target);
        }
        else {
            let mut stop = false;
            for _ in 0..2 {
                if !unnecessary_books.is_empty() {
                    unnecessary_books.pop();
                    continue;
                }
                if !necessary_books.is_empty() {
                    let v = necessary_books.iter().next_back().cloned().unwrap();
                    necessary_books.remove(&v);
                    continue;
                }
                stop = true;
                break;
            }
            if stop {
                break;
            }
        }
        ans += 1;
    }

    println!("{}", ans);
}

use std::{io::Write, collections::{BTreeSet}};
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

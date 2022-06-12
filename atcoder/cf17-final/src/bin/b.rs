#[allow(clippy::needless_range_loop)]
#[allow(clippy::collapsible_if)]
fn main() {
    let mut scanner = Scanner::new();
    let mut cnt: HashMap<char, usize> = HashMap::new();
    let s: String = scanner.cin();
    let cs: Vec<char> = s.chars().collect();

    if s.len() == 1 {
        println!("YES");
        return;
    }

    for &c in cs.iter() {
        *cnt.entry(c).or_insert(0) += 1;
    }

    let vs: Vec<usize> = cnt.values().cloned().collect();
    let max = vs.iter().max().unwrap();
    let min = vs.iter().min().unwrap();

    // aa, bb, cc
    if s.len() == 2 {
        if cnt.len() == 1 {
            println!("NO");
            return;
        }
    }

    // aba, aca, bab, bcb, cac, cbc
    if s.len() >= 3 {
        if cnt.len() <= 2 {
            println!("NO");
            return;
        }
    }

    if max - min <= 1 {
        println!("YES");
    }
    else {
        println!("NO");
    }
}


use std::collections::{VecDeque, HashMap};
use std::io::{self, Write};
use std::str::FromStr;

pub struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
impl Scanner {
    fn new() -> Self {
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
    }

    pub fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

pub fn flush() {
    std::io::stdout().flush().unwrap();
}

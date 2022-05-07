#[allow(clippy::needless_range_loop)]
fn solve(gs: &[char], cs: &[char]) -> i64 {
    let n = cs.len();
    let m = gs.len();

    let mut share_prefix = true;
    for i in 0..n.min(m) {
        if gs[i] < cs[i] {
            return 0;
        }
        if gs[i] != cs[i] {
            share_prefix = false;
        }
    }

    if n > m && share_prefix {
        return 0;
    }

    let mut large_j = None;
    for j in 1..n {
        if 'a' < cs[j] {
            large_j = Some(j);
            break;
        }
    }

    if let Some(j) = large_j {
        if cs[1] < cs[j] && cs[j] > 't' {
            (j - 1) as i64
        }
        else {
            j as i64
        }
    }
    else {
        -1
    }
}


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let t: usize = scanner.cin();

    let gs = vec!['a', 't', 'c', 'o', 'd', 'e', 'r'];

    for _ in 0..t {
        let s: String = scanner.cin();
        let cs: Vec<char> = s.chars().collect();
        println!("{}", solve(&gs, &cs));
    }
}


use std::collections::VecDeque;
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

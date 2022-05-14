#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let mut observed_poems = HashSet::new();
    let mut original_submissions = vec![];

    for i in 0..n {
        let s: String = scanner.cin();
        let t: usize = scanner.cin();

        if observed_poems.contains(&s) {
            continue;
        }

        original_submissions.push((s.clone(), t, i + 1));
        observed_poems.insert(s);
    }

    let mut winner = 0;
    let mut max = 0;
    let k = original_submissions.len();
    for i in 0..k {
        let submission = &original_submissions[i];
        if submission.1 > max {
            winner = submission.2;
            max = submission.1;
        }
    }

    println!("{}", winner);
}


use std::collections::{VecDeque, HashSet};
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

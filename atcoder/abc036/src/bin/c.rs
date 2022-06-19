#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<usize> = (0..n)
        .map(|_| scanner.cin::<usize>())
        .collect();

    let nums: HashSet<usize> = a.iter().cloned().collect();
    let mut nums: Vec<usize> = nums.into_iter().collect();
    nums.sort_unstable();
    let n2i: HashMap<usize, usize> = nums
        .into_iter()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();

    let b: Vec<usize> = a.iter().map(|&ai| n2i[&ai]).collect();
    for bi in b {
        println!("{}", bi);
    }
}

use std::{io::Write, collections::{HashMap, HashSet}};
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

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let x: usize = scanner.cin();
    let y: usize = scanner.cin();
    let z: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);
    let b: Vec<usize> = scanner.vec(n);

    let mut passed = vec![false; n];
    let mut students: Vec<usize> = (0..n).collect();

    students.sort_unstable_by_key(|&x| (Reverse(a[x]), x));
    for i in 0..x {
        passed[students[i]] = true;
    }

    let mut c = 0;
    students.sort_unstable_by_key(|&x| (Reverse(b[x]), x));
    for i in 0..n {
        if c == y {
            break;
        }
        if passed[students[i]] {
            continue;
        }
        passed[students[i]] = true;
        c += 1;
    }

    let mut c = 0;
    students.sort_unstable_by_key(|&x| (Reverse(a[x] + b[x]), x));
    for i in 0..n {
        if c == z {
            break;
        }
        if passed[students[i]] {
            continue;
        }
        passed[students[i]] = true;
        c += 1;
    }

    let passed_students: Vec<usize> = (0..n)
        .filter(|&x| passed[x])
        .map(|x| x + 1)
        .collect();

    for st in passed_students {
        println!("{}", st);
    }
}

use std::{io::Write, cmp::Reverse}; pub fn flush() { std::io::stdout().flush().unwrap(); }
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

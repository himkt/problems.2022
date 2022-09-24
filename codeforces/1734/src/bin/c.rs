fn solve(scanner: &mut Scanner) {
    let size: usize = scanner.cin();
    let bits: Vec<char> = scanner.cin::<String>().chars().collect();
    let mut targets = HashSet::new();
    for (i, c) in bits.iter().enumerate() {
        if c == &'0' {
            targets.insert(i + 1);
        }
    }

    if cfg!(debug_assertions) {
        dbg!(&targets);
    }

    let mut minimum_factors: Vec<usize> = (0..=(size + 1)).collect();

    for p in 1..=size {
        if !targets.contains(&p) {
            continue;
        }
        let mut k = 1;
        while k * p <= size + 1 {
            if !targets.contains(&(k * p)) {
                break;
            }
            minimum_factors[k * p] = minimum_factors[k * p].min(p);
            k += 1;
        }
    }

    if cfg!(debug_assertions) {
        dbg!(&minimum_factors);
        for &p in targets.iter() {
            println!("{}", minimum_factors[p]);
        }
    }
    let mut ans = 0;
    for &p in targets.iter() {
        ans += minimum_factors[p];
    }
    println!("{}", ans);
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let t: usize = scanner.cin();

    for _ in 0..t {
        solve(&mut scanner);
    }
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

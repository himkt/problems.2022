#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut cnt: HashMap<(usize, usize), usize> = HashMap::new();
    for _ in 0..m {
        let k: usize = scanner.cin();
        let x: Vec<usize> = scanner.vec(k);
        for i in 0..k {
            for j in 0..k {
                if i == j {
                    continue;
                }
                cnt.entry((x[i] - 1, x[j] - 1)).and_modify(|e| *e += 1).or_insert(0);
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if !cnt.contains_key(&(i, j)) {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}

use std::{io::Write, collections::HashMap};
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

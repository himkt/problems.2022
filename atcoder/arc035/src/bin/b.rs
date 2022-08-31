use std::collections::HashMap;

const DIV: usize = 1_000_000_007;

fn mod_fac(n: usize) -> usize {
    if n == 1 { return 1; }
    (n * mod_fac(n - 1)) % DIV
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut t: Vec<usize> = (0..n).map(|_| scanner.cin()).collect();
    t.sort_unstable();

    let mut tot_time = 0;
    let mut penalty = 0;
    for &ti in t.iter() {
        tot_time += ti;
        penalty += tot_time;
    }

    let mut cnt = HashMap::new();
    for &ti in t.iter() {
        *cnt.entry(ti).or_insert(0) += 1;
    }

    let mut num = 1;
    for (_, &v) in cnt.iter() {
        num *= mod_fac(v);
        num %= DIV;
    }

    num %= DIV;
    println!("{}", penalty);
    println!("{}", num);
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

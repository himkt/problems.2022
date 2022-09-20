#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let k: usize = scanner.cin();
    let t: usize = scanner.cin();

    let mut cakes: Vec<(usize, usize)> = (0..t)
        .map(|i| {
            let ai = scanner.cin();
            (i, ai)
        })
        .collect();
    cakes.sort_unstable_by_key(|&(_, ai)| ai);

    let mut schedule = vec![0; k];
    let mut cur = 0;

    let mut pos = 1;
    while pos < k {
        schedule[pos] = cakes[cur].0;
        cakes[cur].1 -= 1;
        if cakes[cur].1 == 0 {
            cur += 1;
        }
        pos += 2;
    }

    let mut pos = 0;
    while pos < k {
        schedule[pos] = cakes[cur].0;
        cakes[cur].1 -= 1;
        if cakes[cur].1 == 0 {
            cur += 1;
        }
        pos += 2;
    }

    let ans = (0..(k - 1))
        .filter(|&i| schedule[i] == schedule[i + 1])
        .count();
    println!("{}", ans);
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

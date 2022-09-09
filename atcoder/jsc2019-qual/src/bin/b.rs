const DIV: usize = 1_000_000_007;

fn s(n: usize) -> usize {
    (n * (1 + n)) / 2
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut ans = 0;
    for i in 0..n {
        let mut p = 0;
        for j in 0..=i {
            if a[i] > a[j] {
                p += 1;
            }
        }
        let mut q = 0;
        for j in (i + 1)..n {
            if a[i] > a[j] {
                q += 1;
            }
        }

        ans += (p % DIV) * (s(k - 1) % DIV);
        ans %= DIV;
        ans += (q % DIV) * (s(k) % DIV);
        ans %= DIV;
    }

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

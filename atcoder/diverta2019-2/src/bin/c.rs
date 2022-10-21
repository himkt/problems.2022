#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut a: Vec<i64> = scanner.vec(n);
    a.sort_unstable();
    a.reverse();

    let mut ahead = a[0];
    let mut atail = a[n - 1];

    let mut p = vec![];
    let mut q = vec![];
    for i in 1..(n - 1) {
        if a[i] >= 0 {
            p.push(a[i]);
        }
        else {
            q.push(a[i]);
        }
    }

    let psum: i64 = p.iter().sum::<i64>();
    let qsum: i64 = q.iter().sum::<i64>();
    let n = ahead + psum - qsum - atail;
    println!("{}", n);

    for pi in p {
        println!("{} {}", atail, pi);
        atail -= pi;
    }
    for qi in q {
        println!("{} {}", ahead, qi);
        ahead -= qi;
    }
    println!("{} {}", ahead, atail);
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

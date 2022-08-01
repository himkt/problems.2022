#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut cs: Vec<usize> = vec![0; n - 1];
    let mut ss: Vec<usize> = vec![0; n - 1];
    let mut fs: Vec<usize> = vec![0; n - 1];

    for i in 0..(n - 1) {
        let ci: usize = scanner.cin();
        cs[i] = ci;
        let si: usize = scanner.cin();
        ss[i] = si;
        let fi: usize = scanner.cin();
        fs[i] = fi;
    }

    let mut ans = vec![0; n];
    for s in 0..(n - 1) {
        let mut t = 0;

        for i in s..(n - 1) {
            if t >= ss[i] {
                let v = t - ss[i];
                let d = (v + fs[i] - 1) / fs[i];
                t = ss[i] + d * fs[i];
            }
            else {
                t = ss[i];
            }
            t += cs[i];
        }

        ans[s] = t;
    }

    for ai in ans {
        println!("{}", ai);
    }
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

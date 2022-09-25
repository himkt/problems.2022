fn get_problems(s: usize) -> Vec<usize> {
    if s == 0 {
        return vec![];
    }
    else if s == 1 {
        return vec![1];
    }
    else if s == 2 {
        return vec![2];
    }
    else if s == 3 {
        return vec![1, 2];
    }
    else if s == 4 {
        return vec![4];
    }
    else if s == 5 {
        return vec![1, 4];
    }
    else if s == 6 {
        return vec![2, 4];
    }
    else if s == 7 {
        return vec![1, 2, 4];
    }
    panic!();
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let a: usize = scanner.cin();
    let b: usize = scanner.cin();

    let sa = get_problems(a);
    let sb = get_problems(b);

    let mut ans = 0;
    for si in [1, 2, 4].iter() {
        if sa.contains(si) || sb.contains(si) {
            ans += si;
        }
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

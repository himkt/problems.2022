fn solve(n: usize, k: usize, s: Vec<char>) {
    let num_tot_1 = s.iter().filter(|&x| x == &'1').count();

    let mut cum1d_0: Vec<usize> = vec![0; n + 1];
    let mut cum1d_1: Vec<usize> = vec![0; n + 1];
    for i in 0..n {
        if s[i] == '0' {
            cum1d_0[i + 1] = 1;
        }
        if s[i] == '1' {
            cum1d_1[i + 1] = 1;
        }
    }
    for i in 1..=n {
        cum1d_0[i] += cum1d_0[i - 1];
        cum1d_1[i] += cum1d_1[i - 1];
    }
    debug!("cum1d={:?}", cum1d_0);

    let mut num_valid = 0;
    for i in 0..n {
        if i + k > n {
            continue;
        }
        let num_0 = cum1d_0[i + k] - cum1d_0[i];
        let num_1 = cum1d_1[i + k] - cum1d_1[i];
        if num_0 == 0 && num_1 == num_tot_1 {
            num_valid += 1;
        }
    }

    println!("{}", if num_valid == 1 { "Yes" } else { "No" });
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let t: usize = scanner.cin();
    for _ in 0..t {
        let n: usize = scanner.cin();
        let k: usize = scanner.cin();
        let s: Vec<char> = scanner.cin::<String>().chars().collect();
        solve(n, k, s);
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

#[macro_export]
macro_rules! debug {
    () => {
        #[cfg(debug_assertions)]
        println!();
    };
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}

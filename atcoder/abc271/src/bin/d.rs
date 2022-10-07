#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let s: usize = scanner.cin();

    let mut a = vec![];
    let mut b = vec![];
    for _ in 0..n {
        let ai: usize = scanner.cin();
        let bi: usize = scanner.cin();
        a.push(ai);
        b.push(bi);
    }

    let mut backptr = vec![
        vec![vec![]; s + 1]; n + 1
    ];

    let mut dp = vec![
        vec![false; s + 1]; n + 1
    ];
    dp[0][0] = true;

    for i in 1..=n {
        for j in 0..=s {
            if j >= a[i - 1] && dp[i - 1][j - a[i - 1]] {
                dp[i][j] = true;
                backptr[i][j].push((0, j - a[i - 1]));
            }
            if j >= b[i - 1] && dp[i - 1][j - b[i - 1]] {
                dp[i][j] = true;
                backptr[i][j].push((1, j - b[i - 1]));
            }
        }
    }

    if cfg!(debug_assertions) {
        for row in &dp {
            println!("{:?}", row);
        }
        for row in &backptr {
            println!("{:?}", row);
        }
    }

    if !dp[n][s] {
        println!("No");
        return;
    }

    let mut hist = vec![];

    let (mut flag, mut next) = backptr[n][s][0];
    hist.push(flag);
    if cfg!(debug_assertions) {
        println!("{}", flag);
    }

    for i in (1..n).rev() {
        flag = backptr[i][next][0].0;
        next = backptr[i][next][0].1;

        if cfg!(debug_assertions) {
            println!("{}", flag);
        }
        hist.push(flag);
    }

    println!("Yes");
    for &elem in hist.iter().rev() {
        print!("{}", if elem == 0 { 'H' } else { 'T' });
    }
    println!();
}

use std::{io::Write};
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

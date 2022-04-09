fn f(l: usize, mut x: usize, tot: Vec<usize>, patty: Vec<usize>) -> usize {
    if l == 0 { return 1; }

    if x < 1  {
        return 0;
    }
    x -= 1;

    if x < tot[l-1] {
        return f(l-1, x, tot, patty);
    }
    x -= tot[l-1];

    if x < 1 {
        return patty[l-1] + 1;
    }
    x -= 1;

    if x < tot[l-1] {
        return patty[l-1] + 1 + f(l-1, x, tot, patty);
    }

    patty[l]
}


fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let x: usize = scanner.cin();

    let mut tot: Vec<usize> = vec![0; 51];
    tot[0] = 1;

    for t in 1..=50 {
        tot[t] = 2*tot[t-1] + 3;
    }

    let mut patty: Vec<usize> = vec![0; 51];
    patty[0] = 1;

    for t in 1..=50 {
        patty[t] = 2*patty[t-1] + 1;
    }

    let ans = f(n, x-1, tot, patty);
    println!("{}", ans);
}


use std::collections::VecDeque;
use std::io::{self, Write};
use std::str::FromStr;

#[allow(dead_code)]
struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
#[allow(dead_code)]
impl Scanner {
    fn new() -> Self {
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
    }

    fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

#[allow(dead_code)]
fn flush() {
    std::io::stdout().flush().unwrap();
}

#[macro_export]
macro_rules! trace {
    ($x:expr) => {
        #[cfg(debug_assertions)]
        eprintln!(">>> {} = {:?}", stringify!($x), $x)
    };
    ($($xs:expr),*) => { trace!(($($xs),*)) }
}

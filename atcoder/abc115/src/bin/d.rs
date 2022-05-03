fn patties(n: usize, x: usize, ls: Vec<usize>, ps: Vec<usize>) -> usize {
    if n == 0 { return 1; }
    if x == 1 { return 0; }

    if 1 < x && x <= ls[n-1] + 1 {
        return patties(n-1, x-1, ls, ps);
    }

    if x == ls[n-1] + 2 {
        return 1 + ps[n-1];
    }

    if ls[n-1] + 2 < x {
        return 1 + ps[n-1] + patties(n-1, x-ls[n-1]-2, ls, ps);
    }

    0
}


fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let x: usize = scanner.cin();

    let mut ls: Vec<usize> = vec![1; n+1];
    let mut ps: Vec<usize> = vec![1; n+1];

    for i in 0..n {
        ls[i+1] = 2 * ls[i] + 3;
        ps[i+1] = 2 * ps[i] + 1;
    }

    let ans = patties(n, x, ls, ps);
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

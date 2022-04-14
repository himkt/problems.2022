fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let mut nvs: Vec<i64> = vec![];
    let mut rvs: Vec<i64> = vec![];

    for _ in 0..n {
        let x: i64 = scanner.cin();
        let y: i64 = scanner.cin();
        nvs.push(x+y);
        rvs.push(y-x);
    }

    nvs.sort_unstable();
    rvs.sort_unstable();

    let ans1 = nvs.last().unwrap() - nvs.first().unwrap();
    let ans2 = rvs.last().unwrap() - rvs.first().unwrap();
    let ans = ans1.max(ans2);
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

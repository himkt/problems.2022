const DIV: i64 = 1_000_000_007;


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut b: Vec<i64> = vec![0; n];
    for _ in 0..m {
        let ai: usize = scanner.cin();
        let ai = ai - 1;
        b[ai] = -1;
    }

    if n == 1 && m == 0 {
        println!("1");
        return;
    }

    if b[0] != -1 { b[0] = 1; }
    if b[1] != -1 { b[1] = 1; }

    for i in 0..n {
        // println!("i={}, {:?}", i, b);
        if b[i] == -1 { continue; }

        if i+1 < n && b[i+1] != -1 {
            b[i+1] += b[i];
            b[i+1] %= DIV;
        }
        if i+2 < n && b[i+2] != -1 {
            b[i+2] += b[i];
            b[i+2] %= DIV;
        }
    }

    let ans = b[n-1] % DIV;
    println!("{:?}", ans);
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

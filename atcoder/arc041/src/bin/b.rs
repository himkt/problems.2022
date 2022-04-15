#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut a: Vec<Vec<usize>> = vec![vec![0; m]; n];
    let mut b: Vec<Vec<usize>> = vec![vec![0; m]; n];

    for i in 0..n {
        let cs: String = scanner.cin();
        let vs: Vec<usize> = cs
            .chars()
            .map(|x| x.to_string().parse().unwrap())
            .collect();

        a[i] = vs;
    }

    for i in 0..n {
        for j in 0..m {
            if a[i][j] > 0 {
                let k = a[i][j];
                b[i+1][j] += k;

                a[i][j] -= k;
                a[i+1][j-1] -= k;
                a[i+1][j+1] -= k;
                a[i+2][j] -= k;
            }
        }
    }

    for bi in b.iter() {
        let sbi: String = bi.iter().map(|x| x.to_string()).collect();
        println!("{}", sbi);
    }
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

#[allow(clippy::comparison_chain)]
fn main() {
    let mut scanner = Scanner::new();
    let q: usize = scanner.cin();
    let mut deque = VecDeque::new();

    for _ in 0..q {
        let qtype: usize = scanner.cin();

        if qtype == 1 {
            let x: usize = scanner.cin();
            let c: usize = scanner.cin();
            deque.push_back((x, c));
        }
        else {
            let mut c: usize = scanner.cin();
            let mut ans = 0;

            while c != 0 && !deque.is_empty() {
                // println!("deque={:?}, ans={}", deque, ans);
                let (nx, nc) = deque[0];

                // println!("c={}, nc={}, deque={:?}, deque[0]={:?}", c, nc, deque, deque[0]);

                // c - num > 0
                if nc == c {
                    deque.pop_front();
                    ans += nc * nx;
                }
                else if nc > c {
                    deque[0].1 -= c;
                    ans += c * nx;
                    c = 0;
                }
                else {
                    deque.pop_front();
                    ans += nc * nx;
                    c -= nc;
                }
            }

            println!("{}", ans);
        }
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

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let mut notebook = HashMap::new();

    let mut hist = vec![(0, -1)];
    let mut cursor = 0;

    let q: usize = scanner.cin();
    for _ in 0..q {
        let qtype: String = scanner.cin();
        match qtype.as_str() {
            "ADD" => {
                let x: i64 = scanner.cin();
                hist.push((cursor, x));
                cursor = hist.len() - 1;
            },
            "DELETE" => {
                if cursor > 0 {
                    cursor = hist[cursor].0;
                }
            },
            "SAVE" => {
                let x: usize = scanner.cin();
                notebook.insert(x, cursor);
            },
            "LOAD" => {
                let x: usize = scanner.cin::<usize>();
                if let Some(&cur) = notebook.get(&x) {
                    cursor = cur;
                }
                else {
                    cursor = 0;
                }
            },
            _ => panic!(),
        }
        println!("{}", hist[cursor].1);
        debug!("{}, {:?}, cursor={}", qtype, hist, cursor);
    }
}

use std::{io::Write, collections::HashMap};
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

#[macro_export]
macro_rules! ndarray {
    ($x:expr;) => { $x };
    ($x:expr; $size:expr $( , $rest:expr )*) => {
        vec![ndarray!($x; $($rest),*); $size]
    };
}

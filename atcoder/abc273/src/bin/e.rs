#[allow(clippy::needless_range_loop)]
#[allow(clippy::map_entry)]
fn main() {
    let mut scanner = Scanner::new();
    let mut memo = HashMap::new();
    let mut a: Vec<usize> = vec![];

    let mut cache: HashMap<usize, usize> = HashMap::new();
    let mut updated_after_save = false;
    let mut latest = None;

    let q: usize = scanner.cin();
    for _ in 0..q {
        let qi: String = scanner.cin();
        if qi == "DELETE" {
            a.pop();
            updated_after_save = true;
        }
        else {
            let qj: usize = scanner.cin();
            if qi == "ADD" {
                a.push(qj);
                updated_after_save = true;
            }
            else if qi == "SAVE" {
                if !updated_after_save {
                    let v = latest.unwrap();
                    cache.entry(qj).or_insert(v);
                    cache.entry(qj).and_modify(|e| *e = latest.unwrap());
                }
                else {
                    if memo.contains_key(&qj) {
                        memo.entry(qj).and_modify(|e| *e = a.clone());
                    }
                    else {
                        memo.insert(qj, a.clone());
                    }
                    latest = Some(qj);
                    updated_after_save = false;
                }
            }
            else if qi == "LOAD" {
                if let Some(al) = memo.get(&qj) {
                    a = al.clone();
                }
                else {
                    a = vec![];
                }
            }
        }
        if let Some(v) = a.last() {
            println!("{}", v);
        }
        else {
            println!("-1");
        }
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

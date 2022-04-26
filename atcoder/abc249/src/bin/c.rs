pub struct Bitset<T: Copy> {
    curr: u32,
    array: Vec<T>,
    len: usize,
}

impl<T: Copy> Iterator for Bitset<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.curr >= 1<<self.len {
            return None;
        }

        let mut ret = Vec::<T>::new();
        let r_array = self.array.clone();
        for (i, ai) in r_array.iter().enumerate() {
            let patch = self.curr>>i & 1;
            if patch == 1 {
                ret.push(*ai);
            }
        }

        self.curr += 1;
        Some(ret)
    }
}

pub fn bitset<T: Copy>(a: Vec<T>) -> Bitset<T> {
    let len = a.len();
    Bitset { curr: 0, array: a, len }
}


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();
    let css: Vec<Vec<char>> = (0..n).map(|_| {
        let s: String = scanner.cin();
        s.chars().collect()
    }).collect();

    let mut ans = 0;

    let ss: Vec<usize> = (0..n).collect();
    for ids in bitset(ss) {
        let mut counter = HashMap::new();
        for id in ids {
            for &c in &css[id] {
                *counter.entry(c).or_insert(0) += 1;
            }
        }

        // println!("{:?}", counter);
        let mut cur = 0;
        for (_, v) in counter {
            if v == k {
                cur += 1;
            }
        }
        ans = ans.max(cur);
    }

    println!("{}", ans);
}


use std::collections::{VecDeque, HashMap};
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

#[derive(Debug,Clone)]
pub struct RSQ {
    v: Vec<i64>,
}

impl RSQ {
    const SEQ_LEN: usize = 1<<19;
}

impl Default for RSQ {
    fn default() -> Self {
        RSQ::new()
    }
}

impl RSQ {
    pub fn new() -> Self {
        Self {
            v: vec![0; 2*RSQ::SEQ_LEN]
        }
    }

    pub fn add(&mut self, mut index: usize, value: i64) {
        index += RSQ::SEQ_LEN - 1;
        self.v[index] ^= value;

        loop {
            index /= 2;
            if index == 0 { break; }
            self.v[index] = self.v[index*2] ^ self.v[index*2 + 1];
        }
    }

    pub fn sum(&self, mut l: usize, mut r: usize) -> i64 {
        l += RSQ::SEQ_LEN - 1;
        r += RSQ::SEQ_LEN;

        let mut ans = 0;

        while l < r {
            if l % 2 == 1 {
                ans ^= self.v[l];
                l += 1;
            }
            l /= 2;

            if r % 2 == 1 {
                ans ^= self.v[r-1];
                r -= 1;
            }
            r /= 2;
        }

        ans
    }

    pub fn from(v: Vec<i64>) -> Self {
        let mut rsq = RSQ::new();
        for (index, value) in (0..v.len()).zip(v.into_iter()) {
            let idx = RSQ::SEQ_LEN + index;
            rsq.v[idx] = value;
        }

        rsq
    }
}



#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let q: usize = scanner.cin();

    let mut raq = RSQ::new();

    for i in 0..n {
        let ai = scanner.cin();
        raq.add(i+1, ai);
    }

    for _ in 0..q {
        let t: usize = scanner.cin();

        if t == 1 {
            let x: usize = scanner.cin();
            let y: i64 = scanner.cin();

            raq.add(x, y);
        }
        else {
            let x: usize = scanner.cin();
            let y: usize = scanner.cin();

            println!("{}", raq.sum(x, y));
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

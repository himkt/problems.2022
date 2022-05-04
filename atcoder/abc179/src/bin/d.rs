const DIV: i64 = 998244353;


#[derive(Debug,Clone)]
pub struct RAQ {
    v: Vec<i64>,
}

impl RAQ {
    // const SEQ_LEN: usize = 1<<18;
    const SEQ_LEN: usize = 1_000_000;
}

impl Default for RAQ {
    fn default() -> Self {
        RAQ::new()
    }
}

impl RAQ {
    pub fn new() -> Self {
        Self {
            v: vec![0; 2*RAQ::SEQ_LEN]
        }
    }

    pub fn get(&mut self, mut index: usize) -> i64 {
        index += RAQ::SEQ_LEN - 1;

        let mut sum = 0;
        sum += self.v[index];
        sum %= DIV;

        loop {
            index /= 2;
            if index == 0 { break; }
            sum += self.v[index];
            sum %= DIV;
        }

        sum
    }

    pub fn add(&mut self, mut l: usize, mut r: usize, value: i64) {
        l += RAQ::SEQ_LEN - 1;
        r += RAQ::SEQ_LEN;

        while l < r {
            if l % 2 == 1 {
                self.v[l] += value;
                self.v[l] %= DIV;
                l += 1;
            }
            l /= 2;

            if r % 2 == 1 {
                self.v[r-1] += value;
                self.v[r-1] %= DIV;
                r -= 1;
            }
            r /= 2;
        }
    }

    pub fn from(v: Vec<i64>) -> Self {
        let mut rsq = RAQ::new();
        for (index, value) in (0..v.len()).zip(v.into_iter()) {
            let idx = RAQ::SEQ_LEN + index;
            rsq.v[idx] = value;
        }

        rsq
    }
}



#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();

    let s: Vec<(usize, usize)> = (0..k)
        .map(|_| {
            let l: usize = scanner.cin();
            let r: usize = scanner.cin();
            (l, r)
        })
        .collect();

    // println!("s={:?}", s);

    let mut dp = RAQ::new();
    dp.add(1, 1, 1);

    for i in 1..=n {
        // println!("i={}", i);
        for (l, r) in s.iter() {
            let nl = i + l;
            let nr = i + r;
            // println!("- nl={}, nr={}", nl, nr);

            let v = dp.get(i);
            dp.add(nl, nr, v);
        }
    }

    let ans = dp.get(n);
    println!("{}", ans);
}


use std::collections::{VecDeque};
use std::io::{self, Write};
use std::str::FromStr;

pub struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
impl Scanner {
    fn new() -> Self {
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
    }

    pub fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

pub fn flush() {
    std::io::stdout().flush().unwrap();
}

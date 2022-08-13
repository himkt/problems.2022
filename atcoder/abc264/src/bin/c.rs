pub struct Bitset<T: Copy> {
    curr: usize,
    array: Vec<T>,
    len: usize,
}

impl<T: Copy> Iterator for Bitset<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        if self.curr == (1 << self.len) {
            return None;
        }

        let mut ret = Vec::<T>::new();
        for (i, &ai) in self.array.iter().enumerate() {
            if (self.curr >> i & 1) == 1 {
                ret.push(ai);
            }
        }

        self.curr += 1;
        Some(ret)
    }
}

pub fn bitset<T: Copy>(a: Vec<T>) -> Bitset<T> {
    let len = a.len();
    Bitset {
        curr: 0,
        array: a,
        len,
    }
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let h1: usize = scanner.cin();
    let w1: usize = scanner.cin();
    let a: Vec<Vec<usize>> = (0..h1).map(|_| scanner.vec::<usize>(w1)).collect();
    let h2: usize = scanner.cin();
    let w2: usize = scanner.cin();
    let b: Vec<Vec<usize>> = (0..h2).map(|_| scanner.vec::<usize>(w2)).collect();

    let h_ids_all: Vec<usize> = (0..h1).collect();
    let w_ids_all: Vec<usize> = (0..w1).collect();
    for h_ids in bitset(h_ids_all) {
        for w_ids in bitset(w_ids_all.clone()) {
            if h_ids.len() != h2 || w_ids.len() != w2 {
                continue;
            }
            let mut ans = true;
            for (i, &ai) in h_ids.iter().enumerate() {
                for (j, &aj) in w_ids.iter().enumerate() {
                    if a[ai][aj] != b[i][j] {
                        ans = ans && false;
                    }
                }
            }
            if ans {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}

use std::io::Write;
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

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let q: usize = scanner.cin();

    let mut packets: Vec<(usize, usize)> = (0..n)
        .map(|_| {
            let w: usize = scanner.cin();
            let v: usize = scanner.cin();
            (w, v)
        })
        .collect();
    packets.sort_unstable_by_key(|&(_, v)| Reverse(v));

    let mut xs: Vec<(usize, usize)> = scanner
        .vec::<usize>(m)
        .iter()
        .enumerate()
        .map(&|(i, &w)| (w, i))
        .collect();
    xs.sort_unstable_by_key(|&(w, _)| w);

    for _ in 0..q {
        let l: usize = scanner.cin();
        let r: usize = scanner.cin();

        let mut ans = 0;
        let mut used = HashSet::new();

        for i in 0..m {
            let (_, i) = xs[i];
            if l - 1 <= i && i < r {
                used.insert(i);
            }
        }

        for i in 0..n {
            let (w, v) = packets[i];
            for &(x, j) in xs.iter() {
                if x >= w && !used.contains(&j) {
                    ans += v;
                    used.insert(j);
                    break;
                }
            }
        }
        println!("{}", ans);
    }
}

use std::{io::Write, cmp::Reverse, collections::HashSet};
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

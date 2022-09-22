#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut cumi: Vec<(usize, usize)> = (0..h).map(|i| (i, 0)).collect();
    let mut cumj: Vec<(usize, usize)> = (0..w).map(|j| (j, 0)).collect();
    let mut obstacle_positions = HashSet::new();

    for _ in 0..m {
        let i: usize = scanner.cin::<usize>() - 1;
        let j: usize = scanner.cin::<usize>() - 1;

        cumi[i].1 += 1;
        cumj[j].1 += 1;
        obstacle_positions.insert((i, j));
    }

    if cfg!(debug_assertions) {
        println!("{:?}, {:?}", cumi, cumj);
    }

    cumi.sort_unstable_by_key(|&(_, v)| Reverse(v));
    cumj.sort_unstable_by_key(|&(_, v)| Reverse(v));

    let maxi = cumi[0].1;
    let maxj = cumj[0].1;

    let mut candidatei = vec![];
    for i in 0..h {
        if maxi - cumi[i].1 <= 1 {
            candidatei.push(cumi[i]);
        }
    }
    let mut candidatej = vec![];
    for j in 0..w {
        if maxj - cumj[j].1 <= 1 {
            candidatej.push(cumj[j]);
        }
    }

    if cfg!(debug_assertions) {
        println!("cumi={:?}", cumi);
        println!("cumj={:?}", cumj);
        println!("candidatei={:?}", candidatei);
        println!("candidatej={:?}", candidatej);
    }

    let mut candidatei = HashSet::new();
    for k in 0..h {
        let (ni, v) = cumi[k];
        if maxi == v {
            candidatei.insert(ni);
        }
    }
    let mut candidatej = HashSet::new();
    for k in 0..w {
        let (nj, v) = cumj[k];
        if maxj == v {
            candidatej.insert(nj);
        }
    }

    let num_candidates = candidatei.len() * candidatej.len();
    let mut num_positions = 0;
    for &(i, j) in obstacle_positions.iter() {
        if candidatei.contains(&i) && candidatej.contains(&j) {
            num_positions += 1;
        }
    }

    let mut ans = maxi + maxj;
    if num_candidates == num_positions {
        ans -= 1;
    }
    println!("{}", ans);
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

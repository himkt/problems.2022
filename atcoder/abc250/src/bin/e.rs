const INF: usize = 1_000_000_000_000;


pub fn lower_bound(range: std::ops::Range<usize>, prop: &dyn Fn(usize) -> bool) -> usize {
    if prop(range.start) {
        return range.start;
    }

    let mut ng = range.start;
    let mut ok = range.end;

    while ok - ng > 1 {
        let middle = ng + (ok - ng) / 2;
        match prop(middle) {
            true => ok = middle,
            false => ng = middle,
        }
    }

    ok
}


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);
    let b: Vec<usize> = scanner.vec(n);

    let mut id2id = HashMap::new();
    for &ai in a.iter() {
        let current_len = id2id.len();
        id2id.entry(ai).or_insert(current_len);
    }

    let mut na = vec![];
    let mut nb = vec![];
    for i in 0..n {
        let nai = id2id[&a[i]];
        na.push(nai);

        id2id.entry(b[i]).or_insert(INF);
        let nbi = id2id[&b[i]];
        nb.push(nbi);
    }

    let mut amax = vec![na[0]; n];
    let mut bmax = vec![nb[0]; n];
    for i in 1..n {
        amax[i] = amax[i-1].max(na[i]);
        bmax[i] = bmax[i-1].max(nb[i]);
    }

    let mut nalen = vec![0; n];
    let mut nblen = vec![0; n];
    let mut aset = HashSet::new();
    let mut bset = HashSet::new();
    for i in 0..n {
        aset.insert(na[i]);
        bset.insert(nb[i]);
        nalen[i] = aset.len();
        nblen[i] = bset.len();
    }

    let q: usize = scanner.cin();
    for _ in 0..q {
        let x: usize = scanner.cin::<usize>() - 1;
        let y: usize = scanner.cin::<usize>() - 1;

        let lb = lower_bound(
            0..n,
            &|q| nalen[x] <= nblen[q] && amax[x] <= bmax[q]
        );

        let ub = lower_bound(
            0..n,
            &|q| nalen[x] < nblen[q] || amax[x] < bmax[q],
        );

        if lb < ub && lb <= y && y < ub {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
}


use std::collections::{VecDeque, HashMap, HashSet};
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

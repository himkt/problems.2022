const INF: usize = 1_000_000_000_000_000;


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
    let m: usize = scanner.cin();
    let mut h: Vec<usize> = scanner.vec(n);
    let w: Vec<usize> = scanner.vec(m);

    h.sort_unstable();

    let nv = (n / 2) + 1;
    let mut sf = vec![0; nv];
    let mut sb = vec![0; nv];

    for i in 0..(n / 2) {
        sf[i] = h[2 * i + 1] - h[2 * i];
        sb[i+1] = h[2 * i + 2] - h[2 * i + 1];
    }

    let mut csf = vec![0; nv];
    let mut csb = vec![0; nv];

    csf[0] = sf[0];
    for i in 1..nv {
        csf[i] = sf[i] + csf[i - 1];
    }

    csb[nv - 1] = sb[nv - 1];
    for i in (0..nv - 1).rev() {
        csb[i] = sb[i] + csb[i + 1];
    }

    let mut ans = INF;

    for wi in w {
        let l = lower_bound(0..n, &|x| h[x] >= wi);
        let p = l / 2;

        let mut v = 0;

        if p >= 1 {
            v += csf[p - 1];
        }

        if p + 1 < nv {
            v += csb[p + 1];
        }

        if l == n || l % 2 == 1 {
            v += wi - h[l - 1];
        }
        else {
            v += h[l] - wi;
        }

        ans = ans.min(v);
    }

    println!("{}", ans);
}


use std::collections::VecDeque;
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

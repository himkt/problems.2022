const INF: usize = 1_000_000_000_000_000_000;

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
    let k: usize = scanner.cin();

    let edges: Vec<(usize, usize, usize)> = (0..m)
        .map(|_| {
            let ai = scanner.cin::<usize>() - 1;
            let bi = scanner.cin::<usize>() - 1;
            let c = scanner.cin::<usize>();
            (ai, bi, c)
        })
        .collect();

    let e: Vec<usize> = scanner.vec::<usize>(k).iter().map(|&v| v - 1).collect();
    let mut dist = vec![INF; n];
    dist[0] = 0;

    for i in 0..k {
        let (ai, bi, c) = edges[e[i]];
        dist[bi] = dist[bi].min(dist[ai] + c);
    }

    if dist[n - 1] == INF {
        println!("-1");
    }
    else {
        println!("{}", dist[n - 1]);
    }
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

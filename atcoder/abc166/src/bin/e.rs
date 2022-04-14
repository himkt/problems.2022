pub fn lower_bound(range: std::ops::Range<usize>, prop: &dyn Fn(usize) -> bool) -> usize {
    if prop(range.start) {
        range.start
    }
    else {
        let mut ng = range.start;
        let mut ok = range.end;

        while ok - ng > 1 {
            let middle = ng + (ok - ng) / 2;

            if prop(middle) {
                ok = middle;
            }
            else {
                ng = middle;
            }
        }

        ok
    }
}


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<i64> = scanner.vec(n);

    let mut v: Vec<(i64, i64)> = vec![];
    let mut q: Vec<(i64, i64)> = vec![];

    for i in 0..n {
        let i_i64: i64 = (i + 1) as i64;
        v.push((i_i64 - a[i], i_i64));
        q.push((i_i64 + a[i], i_i64));
    }

    v.sort_unstable();

    // println!("v={:?}", v);
    // println!("q={:?}", q);
    let mut ans = 0;
    for j in 0..n-1 {
        let query = q[j];
        let query_u = (query.0+1, 0);

        let l = lower_bound(j+1..n, &|x| v[x] >= q[j]);
        let r = lower_bound(j+1..n, &|x| v[x] >  query_u);
        // println!("query={:?}, query_u={:?}, v={:?}, l={}, r={}", query, query_u, v, l, r);
        ans += r - l;
    }

    println!("{}", ans);
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

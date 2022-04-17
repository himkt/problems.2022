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
    let a: Vec<usize> = scanner.vec(n);
    let q: usize = scanner.cin();

    let mut b: Vec<(usize, usize)> = vec![];
    for i in 0..n {
        b.push((a[i], i));
    }

    b.sort_unstable();

    for _ in 0..q {
        let l: usize = scanner.cin();
        let l = l - 1;
        let r: usize = scanner.cin();
        let r = r - 1;

        let x: usize = scanner.cin();

        let lower = lower_bound(0..n, &|idx| b[idx] >= (x, l));
        let upper = lower_bound(0..n, &|idx| b[idx] >  (x, r));
        println!("{}", upper - lower);
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

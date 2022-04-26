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
    let x: String = scanner.cin();
    let m: usize = scanner.cin();

    let x: Vec<usize> = x
        .chars()
        .rev()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect();

    if x.len() == 1 {
        if x[0] > m {
            println!("0");
        }
        else {
            println!("1");
        }

        return;
    }

    let &d = x.iter().max().unwrap();
    let lower = d + 1;
    let upper = 10usize.pow(18) + 1;

    let f = |q: usize| {
        let n: usize = x.len();
        let mut v: usize = 0;

        for k in 0..n {
            let diff = match q.checked_pow(k as u32) {
                Some(qp) => match x[k].checked_mul(qp) {
                    Some(v) => v,
                    _ => return true,
                },
                _ => return true,
            };
            v += diff;

            if v > m { return true; }
        }

        // println!("q={}, v={}, m={}", q, v, m);
        v > m
    };

    let upper = lower_bound(lower..upper, &f);
    let ans = upper - lower;
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

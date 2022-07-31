const MAX: i64 = 1_000_000_001;

fn test(arr: &[i64], c: i64, a: i64, b: i64) -> bool {
    let mut add = 0;
    let mut sub = 0;

    for &ai in arr {
        if ai < c {
            let adiff = c - ai;
            add += (adiff + a - 1) / a;
        }
        else {
            let bdiff = ai - c;
            sub += bdiff / b;
        }
    }

    add > sub
}


pub fn lower_bound(range: std::ops::Range<i64>, prop: &dyn Fn(i64) -> bool) -> i64 {
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
    let a: i64 = scanner.cin();
    let b: i64 = scanner.cin();
    let arr: Vec<i64> = scanner.vec(n);

    let l = lower_bound(0..MAX, &|c| test(&arr, c, a, b));
    let ans = l - 1;
    println!("{}", ans);
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

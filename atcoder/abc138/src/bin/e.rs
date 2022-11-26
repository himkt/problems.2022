#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();
    let t: Vec<char> = scanner.cin::<String>().chars().collect();
    let n = s.len();

    let sset: HashSet<char> = s.iter().cloned().collect();
    for ti in t.iter() {
        if !sset.contains(ti) {
            println!("-1");
            return;
        }
    }

    let mut char2pos = HashMap::new();
    for (i, &si) in s.iter().enumerate() {
        char2pos.entry(si).or_insert_with(Vec::new).push(i);
    }

    let mut ans = 0;
    let mut cursor = 0;
    debug!("cursor={}", cursor);

    for &ti in t.iter() {
        let m: usize = char2pos[&ti].len();
        let data = char2pos.get(&ti).unwrap();

        debug!("[ti={}] cursor={}, data={:?}", ti, cursor, data);
        let lower = lower_bound(0..m, &|x| data[x] >= cursor);
        debug!("lower={}", lower);
        if lower == m {
            debug!("from beginning");
            let lower_retry = data[0];
            ans += (n - cursor) + lower_retry + 1;
            cursor = lower_retry + 1;
            debug!("lower={}", lower_retry);
        }
        else {
            debug!("found");
            ans += data[lower] - cursor + 1;
            cursor = data[lower] + 1;
        }

        debug!("ans={}", ans);
    }

    println!("{}", ans);
}

use std::{io::Write, collections::{HashSet, HashMap}};
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

#[macro_export]
macro_rules! debug {
    () => {
        #[cfg(debug_assertions)]
        println!();
    };
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}

#[macro_export]
// ndarray!(val; *shape)
// ndarray!(val; 1) => [val]
// ndarray!(val; 1, 2) => [[val, val]]
macro_rules! ndarray {
    ($x:expr;) => { $x };
    ($x:expr; $size:expr $( , $rest:expr )*) => {
        vec![ndarray!($x; $($rest),*); $size]
    };
}

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

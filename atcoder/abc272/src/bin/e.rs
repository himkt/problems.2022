fn min(a: &[i128], n: usize, c: i128) -> i128 {
    // println!("c={}", c);
    let mut st = HashSet::new();
    for i in 1..=n {
        let v = a[i - 1] + (i as i128) * c;
        st.insert(v);
    }

    // println!("{:?}", st);
    for i in 0..=1_000_000_000 {
        if !st.contains(&i) {
            return i;
        }
    }

    panic!();
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: i128 = scanner.cin();
    let a: Vec<i128> = scanner.vec(n);

    let mut ans: HashMap<i128, i128> = HashMap::new();
    let mut used = HashSet::new();

    for i in 1..=n {
        if a[i - 1] >= 1 {
            continue;
        }
        if a[i - 1] % i as i128 != 0 {
            continue;
        }

        let c = (0 - a[i - 1]) / (i as i128);

        if c > m {
            continue;
        }

        if used.contains(&c) {
            continue;
        }

        let v = min(&a, n, c);
        // println!("i={}, ai={}, c={}, v={}", i, a[i - 1], c, v);
        ans.entry(c).and_modify(|e| *e = (*e).min(v)).or_insert(v);
        used.insert(c);
    }

    for i in 1..=m {
        if ans.contains_key(&i) {
            println!("{}", ans[&i]);
        }
        else {
            println!("0");
        }
    }
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

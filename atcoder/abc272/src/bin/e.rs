const MAX: i128 = 2 * 100_000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let a: Vec<i128> = scanner.vec(n);

    let mut cnt = HashMap::new();

    for i in 0..n {
        let q = i as i128;
        let d = q + 1;

        let mut ai = a[i];
        let mut c: usize = 0;
        if a[i] < 0 {
            let p = (-a[i] + d as i128 - 1) / d;
            c += p as usize;
        }

        debug!("i={}, ai={}, c={}", i, ai, c);
        if c > 1 {
            ai += (c as i128 - 1) * d;
        }
        if c == 0 {
            ai -= d;
        }

        for j in c..=m {
            ai += d;
            if ai > MAX {
                break;
            }
            debug!("j={}, ai={}", j, ai);
            cnt.entry(j).or_insert_with(HashSet::new);
            cnt.entry(j).and_modify(|e| { let _ = (*e).insert(ai); });
        }
    }

    debug!("cnt={:?}", cnt);
    for mm in 1..=m {
        if cnt.contains_key(&mm) {
            let st = &cnt[&mm];
            for q in 0..=MAX {
                if !st.contains(&q) {
                    println!("{}", q);
                    break;
                }
            }
        }
        else {
            println!("0");
        }
    }
}

use std::{io::Write, collections::{HashMap, HashSet}};
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

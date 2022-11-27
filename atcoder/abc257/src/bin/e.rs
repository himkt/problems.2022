#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let c: Vec<usize> = scanner.vec(9);
    let mut e: Vec<(usize, usize)> = c.iter().enumerate()
        .map(|(i, &x)| (x, i + 1))
        .collect();

    e.sort_unstable_by_key(|&(x, v)| (x, Reverse(v)));
    debug!("{:?}", e);

    let mut nn = n;
    let (min_cost, v) = e[0];
    let nm = nn / min_cost;
    let mut ans = vec![v; nm];
    nn -= nm * min_cost;

    e.sort_unstable_by_key(|&(x, v)| (Reverse(v), x));
    debug!("{:?}", e);

    let cursor = 0;
    for i in 0..nm {
        let mut choice = None;
        for j in cursor..9 {
            let (cost, _) = e[j];
            if nn + min_cost >= cost {
                choice = Some(j);
                break;
            }
        }
        if let Some(k) = choice {
            let (cost, vv) = e[k];
            ans[i] = vv;
            nn = nn + min_cost - cost;
        }
        else {
            break;
        }
    }

    let ans: String = ans.iter().map(|&x| x.to_string()).collect();
    println!("{}", ans);
}

use std::{io::Write, cmp::Reverse};
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

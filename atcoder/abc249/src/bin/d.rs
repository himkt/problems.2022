fn sieve(n: usize) -> Vec<usize> {
    let mut ret = vec![];

    let mut k = 2;
    while k * k <= n {
        if n % k == 0 {
            ret.push(k);
            ret.push(n / k);
        }

        k += 1;
    }

    let ret: HashSet<usize> = ret.into_iter().collect();
    let mut ret: Vec<usize> = ret.into_iter().collect();

    ret.sort_unstable();
    ret
}


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut cnt: HashMap<usize, usize> = HashMap::new();
    for &ai in a.iter() {
        *cnt.entry(ai).or_insert(0) += 1;
    }

    let mut ans = 0;

    for &ai in a.iter() {
        let factors = sieve(ai);

        if cnt.contains_key(&1) && cnt.contains_key(&ai) {
            ans += 2 * cnt[&1] * cnt[&ai];
            if ai == 1 {
                ans -= cnt[&1] * cnt[&ai];
            }
        }

        for factor in factors {
            let counterpart = ai / factor;

            // skip e.g. factor=3, counterpart=2
            if counterpart < factor {
                continue;
            }

            if cnt.contains_key(&factor) && cnt.contains_key(&counterpart) {
                ans += 2 * cnt[&factor] * cnt[&counterpart];
                if factor == counterpart {
                    ans -= cnt[&factor] * cnt[&counterpart];
                }
            }
        }
    }

    println!("{}", ans);
}


use std::collections::{VecDeque, HashMap, HashSet};
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

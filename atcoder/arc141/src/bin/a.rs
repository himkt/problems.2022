fn get_factors(n: usize) -> Vec<usize> {
    let limit = (n as f64).sqrt() as usize + 1;
    let mut ret = HashSet::new();

    for p in 1..=limit {
        if n % p == 0 {
            if n != p {
                ret.insert(p);
            }
            if n / p != n {
                ret.insert(n / p);
            }
        }
    }

    let mut vs: Vec<usize> = ret.into_iter().collect();
    vs.sort_unstable();
    vs
}


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let t: usize = scanner.cin();

    for _ in 0..t {
        let s: String = scanner.cin();
        let us: usize = s.parse::<usize>().unwrap();
        let cs: Vec<char> = s.chars().collect();
        let n = cs.len();
        let period_lens = get_factors(n);

        let mut minimum: String = String::from("");
        for _ in 0..(n - 1) {
            minimum += "9";
        }
        let mut ans = minimum.parse::<usize>().unwrap();

        for period in period_lens {
            let slice = &cs[0..period];
            let s: String = slice.iter().collect();
            let v = s.parse::<usize>().unwrap();
            let num_repeats = n / period;

            // 1. repeat v
            let v_string1 = v.to_string();
            let mut candidate1 = String::from("");
            for _ in 0..num_repeats {
                candidate1 += &v_string1;
            }
            let candidate1: usize = candidate1.parse::<usize>().unwrap();

            // 2. repeat v-1
            let v_string2 = (v - 1).to_string();
            let mut candidate2 = String::from("");
            for _ in 0..num_repeats {
                candidate2 += &v_string2;
            }
            let candidate2: usize = candidate2.parse::<usize>().unwrap();

            if candidate1 <= us {
                ans = ans.max(candidate1);
            }

            if candidate2 <= us {
                ans = ans.max(candidate2);
            }
        }

        println!("{}", ans);
    }
}


use std::collections::{VecDeque, HashSet};
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

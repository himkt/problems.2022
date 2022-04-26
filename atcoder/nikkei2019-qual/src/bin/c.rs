#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let mut avs: Vec<i64> = vec![0; n];
    let mut bvs: Vec<i64> = vec![0; n];

    let mut diff_s: Vec<(i64, i64, usize)> = vec![(0, 0, 0); n];
    let mut diff_t: Vec<(i64, i64, usize)> = vec![(0, 0, 0); n];

    for i in 0..n {
        let a: i64 = scanner.cin();
        let b: i64 = scanner.cin();
        avs[i] = a;
        bvs[i] = b;

        diff_s[i] = (a + b, a, i);
        diff_t[i] = (a + b, b, i)
    }

    diff_s.sort_by_key(|&(v, a, _)| (Reverse(v), Reverse(a)));
    diff_t.sort_by_key(|&(v, b, _)| (Reverse(v), Reverse(b)));

    let mut score_s = 0;
    let mut score_t = 0;

    let mut cursor_s = 0;
    let mut cursor_t = 0;
    let mut used = vec![false; n];

    loop {
        loop {
            if cursor_s == n { break; }
            let (_, v, i) = diff_s[cursor_s];

            if used[i] {
                cursor_s += 1;
                continue;
            }

            score_s += v;
            used[i] = true;
            cursor_s += 1;
            break;
        }

        loop {
            if cursor_t == n { break; }
            let (_, v, i) = diff_t[cursor_t];

            if used[i] {
                cursor_t += 1;
                continue;
            }

            score_t += v;
            used[i] = true;
            cursor_t += 1;
            break;
        }

        if cursor_s == n && cursor_t == n { break; }
    }


    let ans = score_s - score_t;
    println!("{}", ans);
}


use std::cmp::Reverse;
use std::collections::{VecDeque};
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

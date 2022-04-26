fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut acnt = HashMap::new();
    for &ai in a.iter() {
        *acnt.entry(ai).or_insert(0) += 1;
    }

    let mut ans: usize = 0;
    for ai in a {

        let mut x = 1;
        let mut factors = vec![];

        while x * x <= ai {
            if ai % x == 0 { factors.push(x); }
            x += 1;
        }

        for aj in factors {
            let ak = ai / aj;

            if !acnt.contains_key(&aj) || !acnt.contains_key(&ak) {
                continue;
            }

            ans += 2 * acnt[&aj] * acnt[&ak];
            if aj == ak { ans -= acnt[&aj] * acnt[&ak]; }
        }
    }

    println!("{}", ans);
}


use std::collections::{VecDeque, HashMap};
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

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut cnt: Vec<HashMap<usize, usize>> = vec![HashMap::new(); k];
    for i in 0..n {
        *cnt[i % k].entry(a[i]).or_insert(0) += 1;
    }

    let mut b = a;
    b.sort_unstable();

    for (i, &bi) in b.iter().enumerate() {
        if !cnt[i % k].contains_key(&bi) {
            println!("No");
            return;
        }
        else {
            cnt[i % k].entry(bi).and_modify(|v| *v -= 1);
            if cnt[i % k][&bi] == 0 {
                cnt[i % k].remove(&bi);
            }
        }
    }

    println!("Yes");
}


use std::collections::{VecDeque, HashMap};
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

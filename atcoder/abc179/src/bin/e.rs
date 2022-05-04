#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let x: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut a: Vec<usize> = vec![x];
    let mut pointer: HashMap<usize, usize> = HashMap::new();
    pointer.insert(x, 0);

    let mut l = 0;
    let mut r = 0;

    for k in 0..=n {
        let ak = a[k];
        let ai = (ak * ak) % m;
        if pointer.contains_key(&ai) {
            l = pointer[&ai];
            r = k + 1;
            break;
        }
        a.push(ai);
        pointer.insert(ai, k+1);
    }

    if l == 0 && l == r {
        let mut ans = 0;
        for i in 0..n {
            ans += a[i];
        }
        println!("{}", ans);
        return;
    }

    let mut sum1 = 0;
    for i in 0..l {
        sum1 += a[i];
    }

    let mut sum2 = 0;
    for i in l..r {
        sum2 += a[i];
    }

    let len1 = n - l;
    let p = len1 / (r - l);
    let len2 = p * (r - l);
    let len3 = len1 - len2;

    let mut sum3 = 0;
    for i in 0..len3 {
        sum3 += a[l+i];
    }

    let ans = sum1 + p * sum2 + sum3;
    println!("{}", ans);
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

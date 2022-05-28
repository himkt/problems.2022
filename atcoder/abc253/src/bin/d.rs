pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}


pub fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: usize = scanner.cin();
    let b: usize = scanner.cin();

    if a == 1 || b == 1 {
        println!("0");
        return;
    }

    let all = n * (1 + n) / 2;

    let na = n / a;
    let amax = na * a;

    let nb = n / b;
    let bmax = nb * b;

    let c = lcm(a, b);
    let nc = n / c;
    let cmax = nc * c;

    let diff1 = na * (a + amax) / 2;
    let diff2 = nb * (b + bmax) / 2;
    let diff3 = nc * (c + cmax) / 2;

    if a == b || b > a && b % a == 0 {
        println!("{}", all - diff1);
    }
    else if a > b && a % b == 0 {
        println!("{}", all - diff2);
    }
    else {
        println!("{}", all - diff1 - diff2 + diff3);
    }
}


use std::collections::VecDeque;
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

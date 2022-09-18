#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let mut ti: usize = 1;
    let mut bi: usize = n;
    while ti < bi {
        let mi = (ti + bi) / 2;
        println!("? {} {} {} {}", ti, mi, 1, n);
        scanner.flush();

        let t: usize = scanner.cin();
        let expected = mi - ti + 1;
        if t != expected {
            bi = mi;
        }
        else {
            ti = mi + 1;
        }
        if ti == bi {
            break;
        }
    }

    let mut lj: usize = 1;
    let mut rj: usize = n;
    while lj < rj {
        let mj = (lj + rj) / 2;
        println!("? {} {} {} {}", 1, n, lj, mj);
        scanner.flush();

        let t: usize = scanner.cin();
        let expected = mj - lj + 1;
        if t != expected {
            rj = mj;
        }
        else {
            lj = mj + 1;
        }
    }

    println!("! {} {}", ti, lj);
}

use std::io::Write;
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

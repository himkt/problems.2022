#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();


    let mut a: Vec<usize> = vec![0; n + 1];
    for i in 0..n {
        a[i + 1] = scanner.cin::<usize>();
    }
    let mut b: Vec<usize> = vec![0; n + 1];

    let half = n / 2;
    for i in (half + 1)..=n {
        b[i] = a[i];
    }

    for i in (1..=half).rev() {
        let mut ret = 0;
        let mut k = 2;
        loop {
            let j = i * k;
            if j > n {
                break;
            }
            ret += b[j];
            ret %= 2;
            k += 1;
        }
        if a[i] == ret {
            b[i] = 0;
        }
        else {
            b[i] = 1;
        }
    }


    let mut m = 0;
    for i in 1..=n {
        if b[i] == 1 {
            m += 1;
        }
    }

    println!("{}", m);
    for i in 1..=n {
        if b[i] == 1 {
            println!("{}", i);
        }
    }
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

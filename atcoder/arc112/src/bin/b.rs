#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let b: i128 = scanner.cin();
    let c: i128 = scanner.cin();
    let mut domains = vec![];

    // [引く, -1 して引いて - 1]
    {
        let l = b - (c / 2);
        let r;

        if c == 1 {
            r = b;
        }
        else {
            r = b + ((c - 2) / 2);
        }

        let min = l.min(r);
        let max = l.max(r);
        domains.push((min, max));
    }

    // [-1 して引く, 引いて- 1]
    {
        let l = - (b - ((c - 1) / 2));
        let r = - (b + ((c - 1) / 2));
        let min = l.min(r);
        let max = l.max(r);
        domains.push((min, max));
    }

    domains.sort_unstable_by_key(|&(min, _)| min);
    //println!("{:?}", domains);

    let ans;
    if domains.len() == 1 {
        let (min, max) = domains[0];
        ans = max - min + 1;
    }
    else {
        let (min1, max1) = domains[0];
        let (min2, max2) = domains[1];

        if min1 == min2 && max1 == max2 {
            ans = max1 - min1 + 1;
        }
        else if max1 >= min2 {
            ans = max2 - min1 + 1;
        }
        else {
            ans = (max1 - min1 + 1) + (max2 - min2 + 1);
        }
    }

    println!("{}", ans);
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

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let _: usize = scanner.cin();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();
    let mut t = vec![];

    let mut l = 0;
    let mut necessity_head = 0;
    for &si in s.iter() {
        if si == '(' {
            l += 1;
        }
        else if si == ')' {
            if l > 0 {
                l -= 1;
            }
            else {
                necessity_head += 1;
            }
        }
    }

    let mut r = 0;
    let mut necessity_tail = 0;
    for &si in s.iter().rev() {
        if si == ')' {
            r += 1;
        }
        else if si == '(' {
            if r > 0 {
                r -= 1;
            }
            else {
                necessity_tail += 1;
            }
        }
    }

    // add `(` to the head
    for _ in 0..necessity_head {
        t.push('(');
    }

    t.extend(s.iter());

    // add `)` to the tail
    for _ in 0..necessity_tail {
        t.push(')');
    }

    let ans: String = t.iter().collect();
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

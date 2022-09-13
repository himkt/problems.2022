fn fmin(s: Vec<char>, i: usize) -> String {

    let mut t = vec![];
    for _ in 0..i {
        t.push('d');
    }
    let mut t: String = t.iter().collect();

    let mut substrings = vec![];
    for j in i..s.len() {
        let mut sequence = vec![];

        for k in (i..=j).rev() {
            if s[k] == 'p' {
                sequence.push('d');
            }
            else {
                sequence.push('p');
            }
        }

        for k in (j + 1)..s.len() {
            sequence.push(s[k]);
        }

        let substring: String = sequence.iter().collect();
        substrings.push(substring);
    }

    substrings.sort();
    t += &substrings[0];
    t
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();

    let mut target = None;
    for i in 0..n {
        if s[i] == 'p' {
            target = Some(i);
            break;
        }
    }

    match target {
        Some(i) => {
            let ans = fmin(s, i);
            println!("{}", ans);
        },
        _ => {
            let ans: String = s.iter().collect();
            println!("{}", ans);
        },
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

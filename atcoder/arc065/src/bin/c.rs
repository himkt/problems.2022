fn match_string(s: &[char], q: &[char]) -> bool {
    if s.len() < q.len() {
        return false;
    }

    for i in 0..q.len() {
        if s[i] != q[i] {
            return false;
        }
    }

    true
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let s: Vec<char> = scanner.cin::<String>().chars().rev().collect();
    let n: usize = s.len();

    let words: Vec<Vec<char>> = vec![
        "maerd".chars().collect(),
        "remaerd".chars().collect(),
        "esare".chars().collect(),
        "resare".chars().collect(),
    ];

    let mut cursor = 0;
    for _ in 0..n {
        let substr = &s[cursor..];

        let mut updated = false;
        for word in words.iter() {
            if match_string(substr, word) {
                cursor += word.len();
                if cursor == n {
                    println!("YES");
                    return;
                }
                updated = true;
                break;
            }
        }

        if !updated {
            println!("NO");
            return;
        }
    }

    panic!("Should not reach");
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

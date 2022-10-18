fn hamming(v1: &[char], v2: &[char]) -> usize {
    let mut ret = 0;
    for (si, ti) in v1.iter().zip(v2) {
        if si != ti {
            ret += 1;
        }
    }
    ret
}

fn flip(c: char) -> char {
    if c == '0' { '1' } else { '0' }
}

#[allow(clippy::needless_range_loop)]
#[allow(clippy::comparison_chain)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();
    let t: Vec<char> = scanner.cin::<String>().chars().collect();

    let mut d = vec!['0'; n];
    let mut hamming_s = hamming(&s, &d);
    let mut hamming_t = hamming(&t, &d);
    debug!("{} {}", hamming_s, hamming_t);

    if (hamming_s.max(hamming_t) - hamming_s.min(hamming_t)) % 2 == 1 {
        println!("-1");
        return;
    }

    for i in (0..n).rev() {
        debug!("[hamming] {} {}", hamming_s, hamming_t);

        if hamming_s == hamming_t {
            break;
        }

        // d[i] はゼロなので、そのままでよい
        if s[i] == t[i] {
            continue;
        }

        // s のハミング距離を 1 増やす必要がある
        if hamming_s < hamming_t && s[i] == d[i] {
            d[i] = flip(s[i]);
            hamming_s += 1;
            hamming_t -= 1;
        }
        // t のハミング距離を 1 増やす必要がある
        else if hamming_s > hamming_t && t[i] == d[i] {
            d[i] = flip(t[i]);
            hamming_s -= 1;
            hamming_t += 1;
        }

        debug!("[iter] {:?}", d);
    }

    debug!("[hamming] {} {}", hamming(&s, &d), hamming(&t, &d));

    let ans: String = d.iter().collect();
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

#[macro_export]
macro_rules! debug {
    () => {
        #[cfg(debug_assertions)]
        println!();
    };
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}

#[macro_export]
macro_rules! ndarray {
    ($x:expr;) => { $x };
    ($x:expr; $size:expr $( , $rest:expr )*) => {
        vec![ndarray!($x; $($rest),*); $size]
    };
}

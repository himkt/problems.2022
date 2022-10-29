#[allow(clippy::needless_range_loop)]
#[allow(clippy::collapsible_if)]
fn main() {
    let mut scanner = Scanner::new();
    let s: Vec<Vec<char>> = (0..9)
        .map(|_| {
            scanner.cin::<String>().chars().collect()
        })
        .collect();

    let mut set = HashSet::new();
    for i in 0i64..9 {
        for j in 0i64..9 {
            for k in 0..9 {
                for m in 0..9 {
                    let p1 = (i, j);
                    let p2 = (k, m);
                    if p1 == p2 {
                        continue;
                    }
                    if s[p1.0 as usize][p1.1 as usize] != '#' {
                        continue;
                    }
                    if s[p2.0 as usize][p2.1 as usize] != '#' {
                        continue;
                    }
                    let dx: i64 = (i - k).abs();
                    let dy: i64 = (j - m).abs();
                    let p3 = (p2.0 + dy, p2.1 + dx);
                    let p4 = (p3.0 + dx, p3.1 - dy);
                    if p3.0 < 0 || 9 <= p3.0 {
                        continue;
                    }
                    if p3.1 < 0 || 9 <= p3.1 {
                        continue;
                    }
                    if p4.0 < 0 || 9 <= p4.0 {
                        continue;
                    }
                    if p4.1 < 0 || 9 <= p4.1 {
                        continue;
                    }
                    let v1 = (p4.0 - p1.0, p4.1 - p1.1);
                    let v2 = (p2.0 - p1.0, p2.1 - p1.1);
                    let inner = v1.0 * v2.0 + v1.1 * v2.1;

                    if s[p1.0 as usize][p1.1 as usize] == '#'
                        && s[p2.0 as usize][p2.1 as usize] == '#'
                        && s[p3.0 as usize][p3.1 as usize] == '#'
                        && s[p4.0 as usize][p4.1 as usize] == '#'
                        && inner == 0 {
                        let mut points = vec![p1, p2, p3, p4];
                        points.sort_unstable();
                        let st: HashSet<&(i64, i64)> = points.iter().collect();
                        if st.len() != 4 {
                            continue;
                        }
                        set.insert(points);
                    }
                }
            }
        }
    }

    println!("{}", set.len());
}

use std::{io::Write, collections::HashSet};
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

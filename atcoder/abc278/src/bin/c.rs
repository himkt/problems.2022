#[allow(clippy::needless_range_loop)]
#[allow(clippy::collapsible_if)]
fn main() {
    let mut scanner = Scanner::new();
    let _: usize = scanner.cin();
    let q: usize = scanner.cin();

    let mut follow_relationships: HashMap<usize, HashSet<usize>> = HashMap::new();
    for _ in 0..q {
        let ti: usize = scanner.cin();
        let ai: usize = scanner.cin();
        let bi: usize = scanner.cin();
        match ti {
            1 => {
                follow_relationships
                    .entry(ai)
                    .or_insert_with(HashSet::new)
                    .insert(bi);
            },
            2 => {
                follow_relationships.entry(ai)
                    .and_modify(|st| { let _ = st.remove(&bi); });
            },
            3 => {
                if follow_relationships.contains_key(&ai) && follow_relationships.contains_key(&bi) {
                    if follow_relationships[&ai].contains(&bi) && follow_relationships[&bi].contains(&ai) {
                        println!("Yes");
                        continue;
                    }
                }
                println!("No");
            },
            _ => {},
        }
    }
}

use std::{io::Write, collections::{HashMap, HashSet}};
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

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut hsh: HashMap<usize, BTreeMap<usize, Vec<usize>>> = HashMap::new();
    for i in 0..n {
        let m: usize = scanner.cin();
        for _ in 0..m {
            let p: usize = scanner.cin();
            let e: usize = scanner.cin();
            let k = hsh.entry(p).or_insert(BTreeMap::new());
            (*k).entry(e).or_insert(Vec::new()).push(i);
        }
    }

    let mut divs = vec![vec![]; n];
    for (p, mp) in hsh {
        let (&top, arr) = mp.range(..).rev().next().unwrap();
        if arr.len() > 1 {
            continue;
        }
        if let Some((&scn, _)) = mp.range(..top).rev().next() {
            let power = top - scn;
            divs[arr[0]].push(format!("{}-{}", p, power));
        }
        else {
            divs[arr[0]].push(format!("{}-{}", p, top));
        }
    }
    let mut st = HashSet::new();
    for div in divs {
        st.insert(div);
    }
    println!("{}", st.len());
}

use std::{io::Write, collections::{BTreeMap, HashMap, HashSet}};
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
// ndarray!(val; *shape)
// ndarray!(val; 1) => [val]
// ndarray!(val; 1, 2) => [[val, val]]
macro_rules! ndarray {
    ($x:expr;) => { $x };
    ($x:expr; $size:expr $( , $rest:expr )*) => {
        vec![ndarray!($x; $($rest),*); $size]
    };
}

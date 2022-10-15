const VS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn check(s1: &[char], s2: &[char], s3: &[char], mp: &HashMap<char, char>) -> bool {
    let s1s: String = s1.iter().map(|si| mp[si]).collect();
    let s2s: String = s2.iter().map(|si| mp[si]).collect();
    let s3s: String = s3.iter().map(|si| mp[si]).collect();
    let v1 = s1s.parse::<usize>().unwrap();
    let v2 = s2s.parse::<usize>().unwrap();
    let v3 = s3s.parse::<usize>().unwrap();

    if mp[&s1[0]] == '0' || mp[&s2[0]] == '0' || mp[&s3[0]] == '0' {
        return false;
    }

    if v1 + v2 != v3 {
        return false;
    }

    println!("{}", s1s);
    println!("{}", s2s);
    println!("{}", s3s);
    true
}

struct Solver {
    s1: Vec<char>,
    s2: Vec<char>,
    s3: Vec<char>,
    cs: Vec<char>,
    mp: HashMap<char, char>,
    st: HashSet<char>,
}

#[allow(clippy::needless_range_loop)]
impl Solver {
    pub fn solve(&mut self, cur: usize) -> bool {
        debug!("[ini] cur={}, st={:?}, mp={:?}", cur, self.st, self.mp);

        if self.mp.len() == self.cs.len() {
            return check(&self.s1, &self.s2, &self.s3, &self.mp);
        }

        let target = self.cs[cur];
        for &nc in VS.iter() {
            if self.st.contains(&nc) {
                continue;
            }

            self.st.insert(nc);  // lock
            self.mp.insert(target, nc);

            if self.solve(cur + 1) {
                return true;
            }

            self.mp.remove(&target);
            self.st.remove(&nc);  // release
        }

        debug!("[fn] cur={}, st={:?}, mp={:?}", cur, self.st, self.mp);
        false
    }
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let s1: Vec<char> = scanner.cin::<String>().chars().collect();
    let s2: Vec<char> = scanner.cin::<String>().chars().collect();
    let s3: Vec<char> = scanner.cin::<String>().chars().collect();

    let mut alphabet: HashSet<char> = HashSet::new();
    alphabet.extend(s1.iter());
    alphabet.extend(s2.iter());
    alphabet.extend(s3.iter());

    // MEMO (himkt):
    // This construction causes an output inconsistenty
    // If you want a consistent output, use BTreeSet or
    // alphabet.sort_unstable() after creating this vector.
    let alphabet: Vec<char> = alphabet.into_iter().collect();

    if alphabet.len() > 10 {
        println!("UNSOLVABLE");
        return;
    }

    let mp = HashMap::new();
    let st = HashSet::new();
    let mut solver = Solver { s1, s2, s3, cs: alphabet, mp, st };

    if !solver.solve(0) {
        println!("UNSOLVABLE");
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

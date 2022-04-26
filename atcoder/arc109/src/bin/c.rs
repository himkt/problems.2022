fn _rsp(l: char, r: char) -> char {
    if l == 'R' {
        if r == 'R' { return 'R' };
        if r == 'P' { return 'P' };
        if r == 'S' { return 'R' }
    }
    if l == 'P' {
        if r == 'R' { return 'P' };
        if r == 'P' { return 'P' };
        if r == 'S' { return 'S' };
    }
    if l == 'S' {
        if r == 'R' { return 'R' };
        if r == 'P' { return 'S' };
        if r == 'S' { return 'S' };
    }

    'X'
}


struct CachedFunction {
    cache: HashMap<(usize, usize), char>,
    s: Vec<char>,
    n: usize,
}


impl CachedFunction {
    fn new(s: Vec<char>, n: usize) -> Self {
        let cache = HashMap::new();
        Self { cache, s, n }
    }

    fn cache_key(&self, k: usize, i: usize) -> (usize, usize) {
        (k, i % self.n)
    }

    #[allow(clippy::map_entry)]
    fn rsp(&mut self, k: usize, i: usize) -> char {

        if k == 1 {
            let li = (2 * i) % self.n;
            let ri = (2 * i + 1) % self.n;
            return _rsp(self.s[li], self.s[ri]);
        }

        let key_l = self.cache_key(k-1, 2*i);
        let rsp_l = if self.cache.contains_key(&key_l) {
            self.cache[&key_l]
        }
        else {
            let rsp = self.rsp(key_l.0, key_l.1);
            self.cache.insert(key_l, rsp);
            rsp
        };

        let key_r = self.cache_key(k-1, 2*i+1);
        let rsp_r = if self.cache.contains_key(&key_r) {
            self.cache[&key_r]
        }
        else {
            let rsp = self.rsp(key_r.0, key_r.1);
            self.cache.insert(key_r, rsp);
            rsp
        };

        _rsp(rsp_l, rsp_r)
    }
}



#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();
    let s: String = scanner.cin();
    let s: Vec<char> = s.chars().collect();

    if n == 1 {
        println!("{}", s[0]);
        return;
    }

    let mut solver = CachedFunction::new(s, n);
    let ans = solver.rsp(k, 0);
    println!("{}", ans);
}


use std::collections::{VecDeque, HashMap};
use std::io::{self, Write};
use std::str::FromStr;

pub struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
impl Scanner {
    fn new() -> Self {
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
    }

    pub fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

pub fn flush() {
    std::io::stdout().flush().unwrap();
}

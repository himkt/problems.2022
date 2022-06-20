const DIV: usize = 998244353;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let d: Vec<usize> = scanner.vec(n);

    let mut counter: HashMap<usize, usize> = HashMap::new();
    for &di in d.iter() {
        *counter.entry(di).or_insert(0) += 1;
    }

    let mut distances: Vec<usize> = counter.keys().cloned().collect();
    let k = distances.len();
    distances.sort_unstable();

    if k == 1 {
        if distances[0] == 0 && counter[&0] == 1 {
            println!("1");
        }
        else {
            println!("0");
        }
        return;
    }

    // 1 x {0} + n * {1}
    if k == 2 {
        if distances[0] == 0 && counter[&0] == 1 && distances[1] == 1 {
            println!("1");
        }
        else {
            println!("0");
        }
        return;
    }

    // rootless
    if distances[0] != 0 {
        println!("0");
        return;
    }

    // multiple roots
    if counter[&0] > 1 {
        println!("0");
        return;
    }

    // distance(1, 1) must be 0
    if d[0] != 0 {
        println!("0");
        return;
    }

    // discontinuous depth set
    for i in 1..k {
        if distances[i] - distances[i - 1] != 1 {
            println!("0");
            return;
        }
    }

    let mut ans = 1;
    for i in 2..k {
        let mut powv = 1;
        for _ in 0..counter[&i] {
            powv *= counter[&(i - 1)];
            powv %= DIV;
        }
        ans *= powv; 
        ans %= DIV;
    }

    ans %= DIV;
    println!("{}", ans);
}

use std::{io::Write, collections::HashMap};
pub fn flush() { std::io::stdout().flush().unwrap(); }
pub struct Scanner { stdin: std::io::Stdin, buffer: std::collections::VecDeque<String> }
impl Scanner {
    fn new() -> Self {
        Self { stdin: std::io::stdin(), buffer: std::collections::VecDeque::new() }
    }

    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            line.split_whitespace().for_each(|x| self.buffer.push_back(String::from(x)));
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

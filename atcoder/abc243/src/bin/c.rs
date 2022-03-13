#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut xs: Vec<usize> = vec![];
    let mut ys: Vec<usize> = vec![];

    for _ in 0..n {
        let x: usize = scanner.cin();
        let y: usize = scanner.cin();
        xs.push(x);
        ys.push(y);
    }

    let cs: String = scanner.cin();
    let s: Vec<char> = cs.chars().collect();

    let mut points_by_y: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..n {
        let key = ys[i];
        points_by_y.entry(key).or_insert_with(Vec::new).push(i);
    }

    for y in points_by_y.keys() {
        let iids = points_by_y.get(y).unwrap();
        let mut ids = iids.clone();
        ids.sort_by_key(|&id| xs[id]);

        let mut obsert_r = false;
        let mut obsert_l_after_r = false;

        for id in ids {
            if s[id] == 'R' {
                obsert_r = true;
            }

            if s[id] == 'L' && obsert_r {
                obsert_l_after_r = true;
            }
        }

        if obsert_l_after_r {
            println!("Yes");
            return;
        }
    }

    println!("No");
}


use std::collections::{VecDeque, HashMap};
use std::io::{self, Write};
use std::str::FromStr;

#[allow(dead_code)]
struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
#[allow(dead_code)]
impl Scanner {
    fn new() -> Self {
        Self {
            stdin: io::stdin(),
            buffer: VecDeque::new(),
        }
    }

    fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    fn usize1(&mut self) -> usize {
        self.cin::<usize>() - 1
    }

    fn chars(&mut self) -> Vec<char> {
        self.cin::<String>().chars().collect()
    }

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

#[allow(dead_code)]
fn flush() {
    std::io::stdout().flush().unwrap();
}

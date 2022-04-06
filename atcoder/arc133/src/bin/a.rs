fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let a2: HashSet<usize> = a.clone().into_iter().collect();
    let mut a2: Vec<usize> = a2.into_iter().collect();
    a2.sort();

    let mut s: HashSet<usize> = HashSet::new();

    for &ak in a2.iter().rev() {
        s.insert(ak);
        if s.len() > 2 {
            break;
        }
    }

    let mut cursor = 0;
    for &ak in &a {
        if cursor < a2.len() && ak == a2[cursor] {
            cursor += 1;
            continue;
        }

        s.insert(ak);
        if s.len() > 3 {
            break;
        }
    }

    let mut vs = vec![];
    for target in s {
        let v: Vec<usize> = a.clone().into_iter().filter(|&ai| ai != target).collect();
        vs.push(v);
    }

    vs.sort();

    let v = vs.iter().next().unwrap();
    let cs: Vec<String> = v.into_iter().map(|&x| x.to_string()).collect();
    let ans = cs.join(" ");
    println!("{}", ans);
}


use std::collections::{VecDeque, HashSet};
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

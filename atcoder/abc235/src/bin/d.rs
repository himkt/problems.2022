const INF: usize = 1001001001001001;


fn main() {
    let mut scanner = Scanner::new();
    let a: usize = scanner.cin();
    let n: usize = scanner.cin();

    let mut numbers: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut ans: usize = INF;

    queue.push_back((n, 0));
    while !queue.is_empty() {
        let top = queue.pop_front().unwrap();
        let cnum = top.0;
        let iter = top.1;

        if numbers.contains(&cnum) {
            continue;
        }

        if cnum == 1 {
            ans = ans.min(iter);
        }

        numbers.insert(cnum);

        if cnum % a == 0 {
            queue.push_back((cnum / a, iter+1));
        }

        if cnum >= 10 && cnum % 10 != 0 {
            let s: String = cnum.to_string();
            if &s[1..=1] == "0" { continue; }

            let tail = &s[0..1];
            let head = &s[1..s.len()];
            let num: String = head.to_string() + tail;
            let num: usize = num.parse().unwrap();
            queue.push_back((num, iter+1));
        }
    }

    if ans == INF {
        println!("-1");
    }
    else {
        println!("{}", ans);
    }
}


use std::io::{self, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashSet};

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

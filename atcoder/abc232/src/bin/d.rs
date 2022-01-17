#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();

    let mut c = vec![vec!['-'; w]; h];
    for i in 0..h {
        let ci: String = scanner.cin();
        let ci: Vec<char> = ci.chars().collect();
        c[i] = ci;
    }

    let mut queue = VecDeque::new();
    let mut used: Vec<Vec<bool>> = vec![vec![false; w]; h];
    let mut cnts: Vec<Vec<usize>> = vec![vec![0; w]; h];

    queue.push_back((0, 0));
    cnts[0][0] = 0;

    let mut ans: usize = 0;
    let ds: Vec<(usize, usize)> = vec![(0, 1), (1, 0)];

    while !queue.is_empty() {
        let front = queue.pop_back().unwrap();
        if used[front.0][front.1] {
            continue;
        }

        for &(dh, dw) in &ds {
            used[front.0][front.1] = true;

            let nh = front.0 + dh;
            let nw = front.1 + dw;

            if nh >= h || nw >= w {
                continue;
            }

            if used[nh][nw] || c[nh][nw] == '#' {
                continue;
            }

            cnts[nh][nw] = cnts[nh][nw].max(cnts[front.0][front.1] + 1);
            ans = ans.max(cnts[nh][nw]);
            queue.push_front((nh, nw));
        }
    }

    println!("{}", ans + 1);
}


use std::io::{self, Write};
use std::str::FromStr;
use std::collections::VecDeque;

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

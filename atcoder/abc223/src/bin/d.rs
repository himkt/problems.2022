struct TopologicalSort {
    g: Vec<Vec<usize>>,
    deg: Vec<usize>,
}


impl TopologicalSort {
    pub fn new(g: Vec<Vec<usize>>, deg: Vec<usize>) -> Self {
        TopologicalSort { g, deg }
    }

    pub fn sort(&mut self) -> Vec<usize> {
        let mut ans: Vec<usize> = vec![];
        let mut s: BinaryHeap<_> = BinaryHeap::new();

        for v in 0..self.g.len() {
            if self.deg[v] == 0 {
                s.push(Reverse(v));
            }
        }

        while let Some(Reverse(v)) = s.pop() {
            ans.push(v);

            for &nv in self.g[v].iter() {
                if self.deg[nv] == 0 {
                    continue;
                }
                self.deg[nv] -= 1;
                if self.deg[nv] == 0 {
                    s.push(Reverse(nv));
                }
            }
        }

        if ans.len() == self.deg.len() {
            ans
        }
        else {
            vec![]
        }
    }
}




#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();

    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    let mut deg: Vec<usize> = vec![0; n];

    for _ in 0..m {
        let a: usize = scanner.cin();
        let a: usize = a - 1;
        let b: usize = scanner.cin();
        let b: usize = b - 1;

        if g[a].contains(&b) {
            continue;
        }

        g[a].push(b);
        deg[b] += 1;
    }

    let mut topological_sorter = TopologicalSort::new(g, deg);
    let ans = topological_sorter.sort();

    if ans.len() == n {
        let ans: Vec<_> = ans.iter().map(|x| (x+1).to_string()).collect();
        let ans = ans.join(" ");
        println!("{}", ans);
    }
    else {
        println!("-1");
    }

}


use std::cmp::Reverse;
use std::collections::{VecDeque, BinaryHeap};
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

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

#[allow(dead_code)]
fn flush() {
    std::io::stdout().flush().unwrap();
}

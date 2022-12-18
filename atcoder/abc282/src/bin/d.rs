#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let mut graph = vec![vec![]; n];
    for _ in 0..m {
        let u: usize = scanner.cin::<usize>() - 1;
        let v: usize = scanner.cin::<usize>() - 1;
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut q = 0;
    let mut p = 0;
    let mut used = vec![false; n];

    for u in 0..n {
        if used[u] {
            continue;
        }
        let mut colors = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((u, 0));
        used[u] = true;
        colors.insert(u, 0);

        let mut valid = true;
        let mut vals = vec![1, 0];
        let mut num_edges = 0;
        let mut used_edges = HashSet::new();

        while let Some((u, c)) = queue.pop_front() {
            let nc = (c + 1) % 2;
            for &v in graph[u].iter() {
                if colors.contains_key(&v) && colors[&v] == c {
                    valid = false;
                }
                if used[v] {
                    debug!("!! {} -> {}", u, v);
                    if !used_edges.contains(&(u, v)) {
                        num_edges += 1;
                    }
                    used_edges.insert((u, v));
                    used_edges.insert((v, u));
                    continue;
                }
                used[v] = true;
                colors.insert(v, nc);
                vals[nc] += 1;
                num_edges += 1;
                debug!("{} -> {}", u, v);
                used_edges.insert((u, v));
                used_edges.insert((v, u));
                queue.push_back((v, (c + 1) % 2));
            }
        }

        debug!("u={}, ne={}, {:?}", u, num_edges, vals);
        if !valid {
            println!("0");
            return;
        }
        if vals[0] * vals[1] != 0 {
            debug!("{:?}", vals);
            p += (vals[0] * (vals[0] - 1)) / 2;
            p += (vals[1] * (vals[1] - 1)) / 2;
            q += num_edges;
            debug!("p={}, m={}", p, q);
        }
    }

    let z = (n * (n - 1)) / 2;
    let ans = z - p - q;
    println!("{}", ans);
}

use std::{io::Write, collections::{HashMap, VecDeque, HashSet}};
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

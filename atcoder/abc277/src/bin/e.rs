const INF: usize = 1_000_000_000_000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let k: usize = scanner.cin();

    let mut graphs = vec![
        vec![vec![]; n],
        vec![vec![]; n],
    ];
    let mut used = vec![
        HashSet::new(),
        HashSet::new(),
    ];
    let mut dists = vec![
        vec![INF; n],
        vec![INF; n],
    ];

    for _ in 0..m {
        let u: usize = scanner.cin::<usize>() - 1;
        let v: usize = scanner.cin::<usize>() - 1;
        let a: usize = scanner.cin();
        graphs[a][u].push(v);
        graphs[a][v].push(u);
    }
    let st: HashSet<usize> = (0..k).map(|_| scanner.cin::<usize>() - 1).collect();

    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 1));

    while let Some((u, cnt, current_state)) = queue.pop_front() {
        debug!("u={}, cnt={}, current_state={}", u, cnt, current_state);
        if dists[current_state][u] <= cnt {
            continue;
        }

        dists[current_state][u] = cnt;
        for &v in graphs[current_state][u].iter() {
            if cnt + 1 < dists[current_state][v] {
                queue.push_back((v, cnt + 1, current_state));
            }
        }
        if st.contains(&u) && !used[current_state].contains(&u) {
            used[current_state].insert(u);
            let next_state = (current_state + 1) % 2;
            if cnt < dists[next_state][u] {
                queue.push_back((u, cnt, next_state));
            }
        }
    }

    debug!("{:?}", dists);
    let ans = dists[0][n - 1].min(dists[1][n - 1]);
    if ans == INF {
        println!("-1");
    }
    else {
        println!("{}", ans);
    }
}

use std::{io::Write, collections::{HashSet, VecDeque}};
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

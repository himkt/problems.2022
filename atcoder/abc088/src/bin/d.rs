const INF: usize = 1_000_000_000;

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();
    let mut board = vec![vec![]; h];
    for i in 0..h {
        let si: Vec<char> = scanner.cin::<String>().chars().collect();
        board[i] = si;
    }

    let h_i64 = h as i64;
    let w_i64 = w as i64;

    let mut backptr = HashMap::new();
    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
    queue.push_back(((0, 0), 0));

    let di = vec![0, -1, 0, 1];
    let dj = vec![1, 0, -1, 0];

    let mut dist = vec![vec![INF; w]; h];
    dist[0][0] = 0;

    while let Some(((i, j), cost)) = queue.pop_front() {
        for k in 0..4 {
            let ni_i64 = i as i64 + di[k];
            let nj_i64 = j as i64 + dj[k];

            if ni_i64 < 0 {
                continue;
            }
            if nj_i64 < 0 {
                continue;
            }
            if ni_i64 >= h_i64 {
                continue;
            }
            if nj_i64 >= w_i64 {
                continue
            }

            let ni = ni_i64 as usize;
            let nj = nj_i64 as usize;

            if board[ni][nj] == '.' && cost + 1 < dist[ni][nj] {
                dist[ni][nj] = cost + 1;
                backptr.entry((ni, nj)).or_insert((i, j));
                backptr.entry((ni, nj)).and_modify(|e| *e = (i, j));
                queue.push_back(((ni, nj), cost + 1));
            }
        }
    }

    if dist[h - 1][w - 1] == INF {
        println!("-1");
        return;
    }

    let mut used = HashSet::new();
    let (mut ci, mut cj) = (h - 1, w - 1);

    while (ci, cj) != (0, 0) {
        used.insert((ci, cj));
        // (ci, cj) = backptr[&(ci, cj)];  <- CE on AtCoder.
        let nc = backptr[&(ci, cj)];
        ci = nc.0;
        cj = nc.1;
    }

    used.insert((0, 0));

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if used.contains(&(i, j)) {
                continue;
            }
            if board[i][j] == '#' {
                continue;
            }
            ans += 1;
        }
    }
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

fn position2idx(i: usize, j: usize) -> usize {
    1000 * i + j
}

#[allow(clippy::needless_range_loop)]
#[allow(clippy::manual_memcpy)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();
    let mut board = vec![vec!['x'; w]; h];
    for i in 0..h {
        let si: Vec<char> = scanner.cin::<String>().chars().collect();
        for j in 0..w {
            board[i][j] = si[j];
        }
    }

    let mut uf = UnionFind::new(1000 * 1000);

    for i in 0..h {
        for j in 0..w {
            // i - 1, j
            if i > 0 {
                let ni = i - 1;
                let nj = j;
                if board[i][j] != board[ni][nj] {
                    let idx1 = position2idx(i, j);
                    let idx2 = position2idx(ni, nj);
                    uf.unite(idx1, idx2);
                }
            }

            // i, j - 1
            if j > 0 {
                let ni = i;
                let nj = j - 1;
                if board[i][j] != board[ni][nj] {
                    let idx1 = position2idx(i, j);
                    let idx2 = position2idx(ni, nj);
                    uf.unite(idx1, idx2);
                }
            }

            // i + 1, j
            if i + 1 < h {
                let ni = i + 1;
                let nj = j;
                if board[i][j] != board[ni][nj] {
                    let idx1 = position2idx(i, j);
                    let idx2 = position2idx(ni, nj);
                    uf.unite(idx1, idx2);
                }
            }

            // i, j + 1
            if j + 1 < w {
                let ni = i;
                let nj = j + 1;
                if board[i][j] != board[ni][nj] {
                    let idx1 = position2idx(i, j);
                    let idx2 = position2idx(ni, nj);
                    uf.unite(idx1, idx2);
                }
            }
        }
    }

    let mut cnt_b: HashMap<usize, usize> = HashMap::new();
    let mut cnt_w: HashMap<usize, usize> = HashMap::new();
    for i in 0..h {
        for j in 0..w {
            let p = uf.parent(position2idx(i, j));
            let cnt = match board[i][j] {
                '#' => &mut cnt_b,
                '.' => &mut cnt_w,
                _ => panic!(),
            };
            *cnt.entry(p).or_insert(0) += 1;
        }
    }
    debug!("{:?}", cnt_b);
    debug!("{:?}", cnt_w);

    let mut ans: usize = 0;
    for (k, &v1) in cnt_b.iter() {
        if let Some(v2) = cnt_w.get(k) {
            debug!("{}*{}", v1, v2);
            ans += v1 * v2;
        }
    }

    println!("{}", ans);
}

use std::{io::Write, collections::HashMap};
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

#[derive(Debug, Clone)]
pub struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

#[allow(clippy::needless_range_loop)]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            sizes: vec![1usize; n],
        }
    }

    pub fn parent(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            x
        } else {
            self.parents[x] = self.parent(self.parents[x]);
            self.parents[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let mut px = self.parent(x);
        let mut py = self.parent(y);

        if px == py {
            return;
        }

        if self.sizes[px] < self.sizes[py] {
            std::mem::swap(&mut px, &mut py);
        }

        self.sizes[px] += self.sizes[py];
        self.parents[py] = px;
    }

    pub fn size(&mut self, x: usize) -> usize {
        let x = self.parent(x);
        self.sizes[x]
    }
}

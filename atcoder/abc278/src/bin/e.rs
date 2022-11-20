#[allow(clippy::needless_range_loop)]
fn cum(cum2d: &[Vec<Vec<usize>>], n: usize, lh: usize, lw: usize, rh: usize, rw: usize) -> usize {
    let mut ret = 0;
    for x in 1..=n {
        let mut v = cum2d.last().unwrap().last().unwrap()[x];
        let mut rem = 0;
        rem += cum2d[rh][rw][x];
        rem += cum2d[lh][lw][x];
        rem -= cum2d[lh][rw][x];
        rem -= cum2d[rh][lw][x];
        v -= rem;
        if v > 0 {
            ret += 1;
        }
    }
    ret
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();
    let n: usize = scanner.cin();
    let sh: usize = scanner.cin();
    let sw: usize = scanner.cin();
    let a: Vec<Vec<usize>> = (0..h).map(|_| scanner.vec(w)).collect();
    let mut cnt2d = vec![vec![vec![0; n + 1]; w + 1]; h + 1];

    for i in 1..=h {
        for j in 1..=w {
            cnt2d[i][j][a[i - 1][j - 1]] += 1;
        }
    }
    for i in 1..=h {
        for j in 2..=w {
            for x in 1..=n {
                cnt2d[i][j][x] += cnt2d[i][j - 1][x];
            }
        }
    }
    for j in 1..=w {
        for i in 2..=h {
            for x in 1..=n {
                cnt2d[i][j][x] += cnt2d[i - 1][j][x];
            }
        }
    }
    for items in cnt2d.iter() {
        debug!("{:?}", items);
    }
    debug!("w-sw={}, h-sh={}", w - sw, h - sh);

    let mut ret = vec![vec![0; w - sw + 1]; h - sh + 1];
    for ch in sh..=h {
        for cw in sw..=w {
            debug!("({}, {}), ({}, {})", ch - sh, cw - sw, ch, cw);
            let v = cum(&cnt2d, n, ch - sh, cw - sw, ch, cw);
            debug!("=> {}", v);
            ret[ch - sh][cw - sw] = v;
        }
    }

    debug!("{:?}", ret);
    for row in ret {
        let rows: String = row.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
        println!("{}", rows);
    }
}

use std::io::Write;
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

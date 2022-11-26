#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();

    let ss: Vec<Vec<char>> = (0..h).map(|_| {
        scanner.cin::<String>().chars().collect()
    }).collect();
    let ts: Vec<Vec<char>> = (0..h).map(|_| {
        scanner.cin::<String>().chars().collect()
    }).collect();

    let mut ws: Vec<Vec<char>> = vec![vec!['x'; h]; w];
    let mut wt: Vec<Vec<char>> = vec![vec!['x'; h]; w];

    for j in 0..w {
        for i in 0..h {
            ws[j][i] = ss[i][j];
            wt[j][i] = ts[i][j];
        }
    }

    debug!("{:?}", ws);
    debug!("{:?}", wt);
    let sss: Vec<String> = ws.iter().map(|cs| cs.iter().collect::<String>()).collect();
    let tss: Vec<String> = wt.iter().map(|cs| cs.iter().collect::<String>()).collect();
    debug!("{:?}", sss);
    debug!("{:?}", tss);

    let mut cnts = HashMap::new();
    let mut cntt = HashMap::new();
    for (sssi, ttti) in sss.iter().zip(tss.iter()) {
        *cnts.entry(sssi).or_insert(0) += 1;
        *cntt.entry(ttti).or_insert(0) += 1;
    }

    debug!("{:?}", cnts);
    debug!("{:?}", cntt);
    for (k, v) in cnts {
        if !cntt.contains_key(k) {
            println!("No");
            return;
        }
        if v != cntt[k] {
            println!("No");
            return;
        }
    }

    println!("Yes");
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

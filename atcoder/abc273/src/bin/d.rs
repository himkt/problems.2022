#[allow(clippy::needless_range_loop)]
#[allow(clippy::collapsible_else_if)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();
    let rs: usize = scanner.cin::<usize>() - 1;
    let cs: usize = scanner.cin::<usize>() - 1;

    let mut blocks_h: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut blocks_w: HashMap<usize, Vec<usize>> = HashMap::new();
    let n: usize = scanner.cin();
    for _ in 0..n {
        let ri: usize = scanner.cin::<usize>() - 1;
        let ci: usize = scanner.cin::<usize>() - 1;
        blocks_h.entry(ri).or_insert_with(Vec::new).push(ci);
        blocks_w.entry(ci).or_insert_with(Vec::new).push(ri);
    }

    let keys: Vec<usize> = blocks_h.keys().cloned().collect();
    for key in keys {
        blocks_h.entry(key).and_modify(|e| e.sort_unstable());
    }

    let keys: Vec<usize> = blocks_w.keys().cloned().collect();
    for key in keys {
        blocks_w.entry(key).and_modify(|e| e.sort_unstable());
    }

    debug!("blocks_h={:?}", blocks_h);
    debug!("blocks_w={:?}", blocks_w);
    let (mut ci, mut cj) = (rs, cs);

    let q: usize = scanner.cin();
    for _ in 0..q {
        let di: char = scanner.cin();
        let li: usize = scanner.cin::<usize>();
        debug!("[iter] {} {}", ci + 1, cj + 1);

        match di {
            'L' => {
                if let Some(arr) = blocks_h.get(&ci) {
                    let m = arr.len();
                    let upper = upper_bound(
                        0..m,
                        &|x| arr[x] < cj,
                    );

                    if upper == m {
                        if cj >= li {
                            cj -= li;
                        }
                        else {
                            cj = 0;
                        }
                    }
                    else {
                        let nj = arr[upper] + 1;
                        if cj >= li {
                            cj = nj.max(cj - li);
                        }
                        else {
                            cj = nj;
                        }
                    }
                }
                else {
                    if cj >= li {
                        cj -= li;
                    }
                    else {
                        cj = 0;
                    }
                }

            },
            'R' => {
                if let Some(arr) = blocks_h.get(&ci) {
                    let m = arr.len();
                    let lower = lower_bound(
                        0..m,
                        &|x| cj < arr[x],
                    );
                    if lower == m {
                        cj += li;
                        cj = cj.min(w - 1);
                    }
                    else {
                        let nj = cj + li;
                        cj = nj.min(arr[lower] - 1);
                    }
                }
                else {
                    cj += li;
                    cj = cj.min(w - 1);
                }

            },
            'U' => {
                if let Some(arr) = blocks_w.get(&cj) {
                    let m = arr.len();
                    let upper = upper_bound(
                        0..m,
                        &|x| arr[x] < ci,
                    );

                    if upper == m {
                        if ci >= li {
                            ci -= li;
                        }
                        else {
                            ci = 0;
                        }
                    }
                    else {
                        let ni = arr[upper] + 1;
                        if ci >= li {
                            ci = ni.max(ci - li);
                        }
                        else {
                            ci = ni;
                        }
                    }
                }
                else {
                    if ci >= li {
                        ci -= li;
                    }
                    else {
                        ci = 0;
                    }
                }
            },
            'D' => {
                if let Some(arr) = blocks_w.get(&cj) {
                    let m = arr.len();
                    let lower = lower_bound(
                        0..m,
                        &|x| ci < arr[x],
                    );
                    if lower == m {
                        ci += li;
                        ci = ci.min(h - 1);
                    }
                    else {
                        let ni = ci + li;
                        ci = ni.min(arr[lower] - 1);
                    }
                }
                else {
                    ci += li;
                    ci = ci.min(h - 1);
                }

            },
            _   => panic!(),
        }
        println!("{} {}", ci + 1, cj + 1);
    }
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

#[macro_export]
macro_rules! ndarray {
    ($x:expr;) => { $x };
    ($x:expr; $size:expr $( , $rest:expr )*) => {
        vec![ndarray!($x; $($rest),*); $size]
    };
}

pub fn lower_bound(range: std::ops::Range<usize>, prop: &dyn Fn(usize) -> bool) -> usize {
    if prop(range.start) {
        return range.start;
    }

    let mut ng = range.start;
    let mut ok = range.end;

    while ok - ng > 1 {
        let middle = ng + (ok - ng) / 2;
        match prop(middle) {
            true => ok = middle,
            false => ng = middle,
        }
    }

    ok
}

pub fn upper_bound(range: std::ops::Range<usize>, prop: &dyn Fn(usize) -> bool) -> usize {
    if !prop(range.start) {
        return range.end;
    }

    let mut ok = range.start;
    let mut ng = range.end;

    while ng - ok > 1 {
        let middle = ok + (ng - ok) / 2;
        match prop(middle) {
            true => ok = middle,
            false => ng = middle,
        }
    }

    ok
}

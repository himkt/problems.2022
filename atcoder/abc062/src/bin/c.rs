#[allow(clippy::needless_range_loop)]
#[allow(clippy::vec_init_then_push)]
fn main() {
    let mut scanner = Scanner::new();
    let h: usize = scanner.cin();
    let w: usize = scanner.cin();

    if h % 3 == 0 || w % 3 == 0 {
        println!("0");
        return;
    }

    // horizontal
    let p = (w + 2) / 3;
    let q = w - 2 * p;
    let mut ans = h * (p.max(q) - p.min(q));

    let p = w / 3;
    let q = w - 2 * p;
    ans = ans.min(h * (p.max(q) - p.min(q)));

    // vertical
    let p = (h + 2) / 3;
    let q = h - 2 * p;
    ans = ans.min((p.max(q) - p.min(q)) * w);

    let p = h / 3;
    let q = h - 2 * p;
    ans = ans.min((p.max(q) - p.min(q)) * w);

    let ch = h / 2;
    let cw = w / 2;

    // horizontal
    for mh in 0..h {
        let ph = h - mh;
        let pw = w - cw;
        let mut vs = vec![];
        vs.push(mh * w);
        vs.push(ph * cw);
        vs.push(ph * pw);
        vs.sort_unstable();
        ans = ans.min(vs[2] - vs[0]);
    }

    // vertical
    for mw in 0..w {
        let ph = h - ch;
        let pw = w - mw;
        let mut vs = vec![];
        vs.push(h * mw);
        vs.push(ch * pw);
        vs.push(ph * pw);
        vs.sort_unstable();
        ans = ans.min(vs[2] - vs[0]);
    }

    println!("{}", ans);
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

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();
    let n: usize = s.len();
    let t: Vec<(usize, char)> = (0..n)
        .map(|i| (i, s[i]))
        .filter(|&(_, si)| si != 'x')
        .collect();

    if t.is_empty() {
        println!("0");
        return;
    }

    let mut li = 0;
    let mut ri = t.len() - 1;
    while li < ri {
        if t[li].1 != t[ri].1 {
            println!("-1");
            return;
        }

        li += 1;
        ri -= 1;
    }

    if li > ri {
        li -= 1;
        ri += 1;
    }

    let mut ans = 0;

    // axxa  => 0
    // axxxa => 1
    if li != ri && t[ri].0 - t[li].0 % 2 == 0 {
        ans += 1;
    }

    while li > 0 {
        let dl = t[li].0 - t[li - 1].0;
        let dr = t[ri + 1].0 - t[ri].0;
        ans += dl.max(dr) - dl.min(dr);
        li -= 1;
        ri += 1;
    }

    let dl_head = t[li].0;
    let dr_tail = n - 1 - t[ri].0;
    ans += dl_head.max(dr_tail) - dl_head.min(dr_tail);
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

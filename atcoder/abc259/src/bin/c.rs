#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();
    let t: Vec<char> = scanner.cin::<String>().chars().collect();

    if s.len() > t.len() {
        println!("No");
        return;
    }

    let mut cursor = 0;
    let mut ss = vec![];
    ss.push((s[cursor], 1));
    for i in 1..s.len() {
        if s[i] == ss[cursor].0 {
            ss[cursor].1 += 1;
        }
        else {
            ss.push((s[i], 1));
            cursor += 1;
        }
    }

    let mut cursor = 0;
    let mut ts = vec![];
    ts.push((t[cursor], 1));
    for i in 1..t.len() {
        if t[i] == ts[cursor].0 {
            ts[cursor].1 += 1;
        }
        else {
            ts.push((t[i], 1));
            cursor += 1;
        }
    }

    if ss.len() != ts.len() {
        println!("No");
        return;
    }

    for i in 0..ss.len() {
        let (si, sc) = ss[i];
        let (ti, tc) = ts[i];

        if si != ti {
            println!("No");
            return;
        }

        if sc == tc {
            continue;
        }

        if sc > tc || sc < 2 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

use std::io::Write; pub fn flush() { std::io::stdout().flush().unwrap(); }
pub struct Scanner { buffer: std::collections::VecDeque<String>, buf: String }
impl Scanner {
    pub fn new() -> Self { Scanner { buffer: std::collections::VecDeque::new(), buf: String::new() } }
    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        if !self.buffer.is_empty() { return self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap(); }
        self.buf.truncate(0); std::io::stdin().read_line(&mut self.buf).ok();
        self.buf.to_owned().split_whitespace().for_each(|x| self.buffer.push_back(String::from(x)));
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> { (0..n).map(|_| self.cin()).collect() }
}

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

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();
    let w: Vec<usize> = scanner.vec(n);

    let mut wl: Vec<usize> = (0..n).filter(|&i| s[i] == '0').map(|i| w[i]).collect();
    let mut wr: Vec<usize> = (0..n).filter(|&i| s[i] == '1').map(|i| w[i]).collect();
    wl.sort_unstable();
    wr.sort_unstable();

    let nl = wl.len();
    let nr = wr.len();

    if nl == 0 {
        println!("{}", nr);
        return;
    }

    if nr == 0 {
        println!("{}", nl);
        return;
    }

    let mut ans = 0;
    for x in w {
        let li = lower_bound(0..nl, &|q| wl[q] >= x);
        let ri = lower_bound(0..nr, &|q| wr[q] >= x);
        ans = ans.max(li + (nr - ri));
    }

    println!("{}", ans);
}

use std::io::Write;
pub fn flush() { std::io::stdout().flush().unwrap(); }
pub struct Scanner { stdin: std::io::Stdin, buffer: std::collections::VecDeque<String> }
impl Scanner {
    fn new() -> Self {
        Self { stdin: std::io::stdin(), buffer: std::collections::VecDeque::new() }
    }

    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            line.split_whitespace().for_each(|x| self.buffer.push_back(String::from(x)));
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}


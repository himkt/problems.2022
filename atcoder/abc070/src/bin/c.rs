pub fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: i128, b: i128) -> i128 {
    (a / gcd(a, b)) * b
}


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let t: Vec<i128> = scanner.vec(n);

    if n == 1 {
        println!("{}", t[0]);
        return;
    }

    let mut ans = lcm(t[0], t[1]);
    for i in 2..n {
        ans = lcm(ans, t[i]);
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

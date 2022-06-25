fn succ(c: char) -> char {
    if c == 'z' { return 'a'; }
    char::from(c as u8 + 1)
}

fn f(c: char) -> usize {
    let t = 'z' as u8;
    let s = c as u8;
    (t - s + 1) as usize
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let s: Vec<char> = scanner.cin::<String>().chars().collect();
    let mut k: usize = scanner.cin();
    let n: usize = s.len();

    let mut result = vec![];
    for c in s {
        if c == 'a' {
            result.push('a');
            continue;
        }

        let cost = f(c);

        if cost <= k {
            result.push('a');
            k -= cost;

        }
        else {
            result.push(c);
        }
    }

    for _ in 0..k {
        result[n - 1] = succ(result[n - 1]);
    }
    
    let ans: String = result.iter().collect();
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

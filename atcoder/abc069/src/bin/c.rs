#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut num_xx: usize = 0;
    let mut num_2x: usize = 0;
    let mut num_4x: usize = 0;

    for ai in a {
        if ai % 4 == 0 {
            num_4x += 1;
        }
        else if ai % 2 == 0 {
            num_2x += 1;
        }
        else {
            num_xx += 1;
        }
    }

    num_xx += num_2x % 2;
    if num_xx == 0 {
        println!("Yes");
    }
    else if num_4x > 0 && num_4x >= num_xx - 1 {
        println!("Yes");
    }
    else {
        println!("No");
    }
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

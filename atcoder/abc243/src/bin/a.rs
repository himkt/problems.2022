fn main() {
    let mut scanner = Scanner::new();
    let mut v: i64 = scanner.cin();
    let a: i64 = scanner.cin();
    let b: i64 = scanner.cin();
    let c: i64 = scanner.cin();

    let mut cursor = 'a';
    loop {
        let k = match cursor {
            'a' => a,
            'b' => b,
            'c' => c,
            _ => panic!()
        };

        if v - k < 0 {
            break;
        }

        v -= k;
        cursor = match cursor {
            'a' => 'b',
            'b' => 'c',
            'c' => 'a',
            _ => panic!()
        }
    }

    let ans = match cursor {
        'a' => 'F',
        'b' => 'M',
        'c' => 'T',
        _ => panic!()
    };

    println!("{}", ans);
}


use std::collections::VecDeque;
use std::io::{self, Write};
use std::str::FromStr;

#[allow(dead_code)]
struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
#[allow(dead_code)]
impl Scanner {
    fn new() -> Self {
        Self {
            stdin: io::stdin(),
            buffer: VecDeque::new(),
        }
    }

    fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    fn i641(&mut self) -> usize {
        self.cin::<usize>() - 1
    }

    fn chars(&mut self) -> Vec<char> {
        self.cin::<String>().chars().collect()
    }

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

#[allow(dead_code)]
fn flush() {
    std::io::stdout().flush().unwrap();
}

#[allow(clippy::many_single_char_names)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let q: usize = scanner.cin();

    let a: Vec<usize> = scanner.vec(n);
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();

    for (i, ai) in a.into_iter().enumerate() {
        (*map.entry(ai).or_insert_with(Vec::new)).push(i);
    }

    for _ in 0..q {
        let x: usize = scanner.cin();
        let y: usize = scanner.cin();
        let y: usize = y - 1;

        let empty_items = vec![];
        let items = map
            .get(&x)
            .or(Some(&empty_items))
            .unwrap();

        if items.len() < y + 1 {
            println!("-1");
        }
        else {
            println!("{}", items[y] + 1);
        }
    }
}


use std::io::{self, Write};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap};

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

    fn usize1(&mut self) -> usize {
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

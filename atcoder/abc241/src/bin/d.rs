use std::collections::BTreeSet;


fn main() {
    let mut scanner = Scanner::new();
    let mut btree_set: BTreeSet<(i64, usize)> = BTreeSet::new();
    let q: usize = scanner.cin();

    for i in 0..q {
        let query_type: usize = scanner.cin();
        let x: i64 = scanner.cin();
        // println!("query_type: {}, btree_set: {:?}", query_type, btree_set);

        match query_type {
            1 => {
                btree_set.insert((x, i));
            },
            2 => {
                let k: usize = scanner.cin();
                let ans = btree_set
                    .range(..=(x, q))
                    .rev()
                    .nth(k-1)
                    .map_or(-1, |&(v, _)| v);

                println!("{}", ans);
            },
            3 => {
                let k: usize = scanner.cin();
                let ans = btree_set
                    .range((x, 0)..)
                    .nth(k-1)
                    .map_or(-1, |v| v.0);

                println!("{}", ans);
            },
            _ => panic!(),
        }
    }
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

    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

#[allow(dead_code)]
fn flush() {
    std::io::stdout().flush().unwrap();
}

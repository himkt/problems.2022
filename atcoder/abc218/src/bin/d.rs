fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut ys_by_x: HashMap<usize, Vec<usize>> = HashMap::new();

    for _ in 0..n {
        let x: usize = scanner.cin();
        let y: usize = scanner.cin();
        ys_by_x.entry(x).or_insert_with(Vec::new).push(y);
    }

    // println!("{:?}", ys_by_x);

    let mut pair_count: HashMap<(usize, usize), usize> = HashMap::new();
    for (_, mut ys) in ys_by_x.into_iter() {
        if ys.len() < 2 {
            continue;
        }
        ys.sort_unstable();

        let m = ys.len();
        for i in 0..m {
            for j in i+1..m {
                *pair_count.entry((ys[i], ys[j])).or_insert(0) += 1;
            }
        }
    }

    // println!("{:?}", pair_count);

    let mut ans = 0;
    for v in pair_count.values() {
        ans += v*(v-1) / 2;
    }

    println!("{}", ans);
}


use std::collections::{VecDeque, HashMap};
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

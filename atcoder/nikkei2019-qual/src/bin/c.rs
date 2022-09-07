#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let mut avs = vec![];
    let mut bvs = vec![];
    for _ in 0..n {
        let ai: i64 = scanner.cin();
        let bi: i64 = scanner.cin();
        avs.push(ai);
        bvs.push(bi);
    }

    let mut a_gain = BinaryHeap::new();
    let mut b_gain = BinaryHeap::new();
    for i in 0..n {
        let ai = avs[i];
        let bi = bvs[i];
        a_gain.push((ai + bi, ai, i));
        b_gain.push((ai + bi, bi, i));
    }

    let mut used = HashSet::new();
    let mut cur = 0;
    let mut ascore = 0;
    let mut bscore = 0;

    while cur < n {
        while let Some((_, _, i)) = a_gain.pop() {
            if used.contains(&i) {
                continue;
            }

            ascore += avs[i];
            used.insert(i);
            cur += 1;
            break;
        }

        while let Some((_, _, i)) = b_gain.pop() {
            if used.contains(&i) {
                continue;
            }

            bscore += bvs[i];
            used.insert(i);
            cur += 1;
            break;
        }
    }

    println!("{}", ascore - bscore);
}


use std::collections::{VecDeque, HashSet, BinaryHeap};
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
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
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

#[macro_export]
macro_rules! trace {
    ($x:expr) => {
        #[cfg(debug_assertions)]
        eprintln!(">>> {} = {:?}", stringify!($x), $x)
    };
    ($($xs:expr),*) => { trace!(($($xs),*)) }
}

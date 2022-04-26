#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut p: Vec<usize> = scanner
        .vec(n)
        .into_iter()
        .map(|x: usize| x-1)
        .collect();

    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut used: Vec<bool> = vec![false; n-1];

    for (i, &pi) in p.iter().enumerate() {
        *map.entry(pi).or_insert(0) = i;
    }

    let mut swapcnt = 0;
    let mut ops = vec![];

    for i in 0..n {
        loop {
            let position = *map.get(&i).unwrap();

            let (l, r) = match position.cmp(&i) {
                Ordering::Equal => break,
                Ordering::Greater => {
                    if used[position-1] { break; }
                    (position-1, position)
                },
                Ordering::Less => {
                    if used[position] { break; }
                    (position, position+1)
                },
            };

            let lv = p[l];
            let rv = p[r];

            *map.entry(lv).or_insert(0) += 1;
            *map.entry(rv).or_insert(0) -= 1;

            p[l] = rv;
            p[r] = lv;

            used[l] = true;
            swapcnt += 1;
            ops.push(r);
        }
    }

    let is_sorted = p
        .iter()
        .enumerate()
        .all(|(i, &pi)| i == pi);

    if is_sorted && swapcnt == n-1 {
        for op in ops {
            println!("{}", op);
        }
    }
    else {
        println!("-1");
    }
}


use std::cmp::Ordering;
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

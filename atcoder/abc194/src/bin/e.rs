#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let m: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut counter: HashMap<usize, usize> = HashMap::new();
    for i in 0..m {
        *counter.entry(a[i]).or_insert(0) += 1;
    }

    let mut ans = 1_500_000;
    for i in 0..=ans {
        if !counter.contains_key(&i) {
            ans = i;
            break;
        }
    }

    for i in m..n {
        *counter.entry(a[i-m]).or_insert(0) -= 1;
        *counter.entry(a[i]).or_insert(0) += 1;

        if counter.get(&a[i-m]).unwrap() == &0 {
            ans = ans.min(a[i-m]);
            counter.remove(&a[i-m]);
        }
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

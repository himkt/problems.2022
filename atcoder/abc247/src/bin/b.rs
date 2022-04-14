fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();

    let mut ss: Vec<_> = vec![];
    let mut ts: Vec<_> = vec![];

    let mut scount: HashMap<_, usize> = HashMap::new();
    let mut tcount: HashMap<_, usize> = HashMap::new();

    for _ in 0..n {
        let s: String = scanner.cin();
        let t: String = scanner.cin();
        *scount.entry(s.clone()).or_insert(0) += 1;
        *tcount.entry(t.clone()).or_insert(0) += 1;

        ss.push(s);
        ts.push(t);
    }

    for i in 0..n {
        let mut s_ok = true;
        let mut t_ok = true;

        for j in 0..n {
            if i == j { continue; }

            if ss[i] == ss[j] {
                s_ok = false;
                break;
            }

            if ss[i] == ts[j] {
                s_ok = false;
                break;
            }
        }

        for j in 0..n {
            if i == j { continue; }

            if ts[i] == ss[j] {
                t_ok = false;
                break;
            }

            if ts[i] == ts[j] {
                t_ok = false;
                break;
            }
        }

        if !(s_ok || t_ok) {
            println!("No");
            return;
        }
    }

    println!("Yes");
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

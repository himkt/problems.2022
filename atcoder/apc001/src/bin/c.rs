const VACANT: &str = "Vacant";
const UNREGI: &str = "NA";


#[allow(clippy::collapsible_if)]
#[allow(clippy::collapsible_else_if)]
#[allow(clippy::if_same_then_else)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let mut seats: Vec<String> = vec![String::from(UNREGI); n];

    println!("0");
    let result: String = scanner.cin();
    flush();

    if result == VACANT {
        return;
    }

    seats[0] = result;

    println!("{}", n - 1);
    let result: String = scanner.cin();
    flush();

    if result == VACANT {
        return;
    }

    seats[n - 1] = result;

    let mut l = 0;
    let mut r = n - 1;

    while r > l {
        let m = l + (r - l) / 2;
        println!("{}", m);

        let result: String = scanner.cin();
        flush();

        if result == VACANT {
            return;
        }

        if seats[l] != UNREGI {
            if (m - l) % 2 == 0 {
                if seats[l] == result {
                    l = m;
                }
                else {
                    r = m;
                }
            }
            else {
                if seats[l] == result {
                    r = m;
                }
                else {
                    l = m;
                }
            }
        }
        else {
            if (r - m) % 2 == 0 {
                if seats[r] == result {
                    r = m;
                }
                else {
                    l = m;
                }
            }
            else {
                if seats[r] == result {
                    l = m;
                }
                else {
                    r = m;
                }
            }
        }

        seats[m] = result;
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

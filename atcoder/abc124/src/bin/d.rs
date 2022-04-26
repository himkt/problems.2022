pub fn lower_bound(range: std::ops::Range<usize>, prop: &dyn Fn(usize) -> bool) -> usize {
    if prop(range.start) {
        range.start
    }
    else {
        let mut ng = range.start;
        let mut ok = range.end;

        while ok - ng > 1 {
            let middle = ng + (ok - ng) / 2;

            if prop(middle) {
                ok = middle;
            }
            else {
                ng = middle;
            }
        }

        ok
    }
}


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();

    let s: String = scanner.cin();
    let cs: Vec<char> = s.chars().collect();

    let mut is: Vec<usize> = vec![0; n];
    if cs[0] == '0' { is[0] += 1; }

    for i in 1..n {
        is[i] = is[i-1];

        if cs[i] == '0' && cs[i-1] == '1' {
            is[i] += 1;
        }
    }

    if n == 1 {
        println!("1");
        return;
    }

    let mut ans = 0;

    for i in 0..n {
        let mut up = is[i];

        if cs[i] == '0' {
            up += k - 1;
        }
        else {
            up += k;
        }

        let r = lower_bound(i..n, &|x| up < is[x]);
        ans = ans.max(r - i);

        // println!("i={}, up={}, {}", i, up, ans);
    }

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

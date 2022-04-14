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


const INF: usize = 100_000_000_000_000;


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let x: usize = scanner.cin();
    let y: usize = scanner.cin();
    let a: Vec<usize> = scanner.vec(n);

    let mut infeasibles = vec![];
    let mut minimums = vec![];
    let mut maximums = vec![];

    for i in 0..n {
        if a[i] == x {
            maximums.push(i);
        }
        if a[i] == y {
            minimums.push(i);
        }
        if a[i] < y || x < a[i] {
            infeasibles.push(i);
        }
    }

    let num_maximums = maximums.len();
    let num_minimums = minimums.len();
    let num_infeasibles = infeasibles.len();

    let mut ans = 0;

    for i in 0..n {
        let least_maximum_idx = if num_maximums == 0 {
            INF
        }
        else {
            let l = lower_bound(0..num_maximums, &|x| maximums[x] >= i);
            if l == num_maximums {
                INF
            }
            else {
                maximums[l]
            }
        };

        let least_minimum_idx = if num_minimums == 0 {
            INF
        }
        else {
            let l = lower_bound(0..num_minimums, &|x| minimums[x] >= i);
            if l == num_minimums {
                INF
            }
            else {
                minimums[l]
            }
        };

        let least_infeasible_idx = if num_infeasibles == 0 {
            INF
        }
        else {
            let l = lower_bound(0..num_infeasibles, &|x| infeasibles[x] >= i);
            if l == num_infeasibles {
                INF
            }
            else {
                infeasibles[l]
            }
        };

        if least_minimum_idx == INF || least_maximum_idx == INF {
            continue;
        }

        if least_minimum_idx < least_infeasible_idx && least_infeasible_idx < least_maximum_idx {
            continue;
        }

        if least_infeasible_idx < least_minimum_idx || least_infeasible_idx < least_maximum_idx {
            continue;
        }

        ans += least_infeasible_idx.min(n) - least_minimum_idx.max(least_maximum_idx);
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

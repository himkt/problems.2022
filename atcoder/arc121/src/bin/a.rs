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
    let mut points = vec![];

    for i in 0..n {
        let x: i64 = scanner.cin();
        let y: i64 = scanner.cin();
        points.push((x, y, i));
    }

    let mut points_by_x = points.clone();
    points_by_x.sort_unstable_by_key(|&(x, _, _)| x);

    let mut points_by_y = points;
    points_by_y.sort_unstable_by_key(|&(_, y, _)| y);

    let mut vs: Vec<(i64, usize, usize)> = vec![
        (points_by_x[n-1].0 - points_by_x[0].0, points_by_x[n - 1].2, points_by_x[0].2),
        (points_by_x[n-1].0 - points_by_x[1].0, points_by_x[n - 1].2, points_by_x[1].2),
        (points_by_x[n-2].0 - points_by_x[0].0, points_by_x[n - 2].2, points_by_x[0].2),
        (points_by_x[n-2].0 - points_by_x[1].0, points_by_x[n - 2].2, points_by_x[1].2),
        (points_by_y[n-1].1 - points_by_y[0].1, points_by_y[n - 1].2, points_by_y[0].2),
        (points_by_y[n-1].1 - points_by_y[1].1, points_by_y[n - 1].2, points_by_y[1].2),
        (points_by_y[n-2].1 - points_by_y[0].1, points_by_y[n - 2].2, points_by_y[0].2),
        (points_by_y[n-2].1 - points_by_y[1].1, points_by_y[n - 2].2, points_by_y[1].2),
    ];

    vs.sort_unstable_by_key(|&(v, _, _)| Reverse(v));

    let prev_pair = (vs[0].1, vs[0].2);

    for (cost, i, j) in vs {
        if prev_pair == (i, j) { continue; }
        println!("{}", cost);
        break;
    }
}


use std::cmp::Reverse;
use std::collections::{VecDeque};
use std::io::{self, Write};
use std::str::FromStr;

pub struct Scanner {
    stdin: io::Stdin,
    buffer: VecDeque<String>,
}
impl Scanner {
    fn new() -> Self {
        Self { stdin: io::stdin(), buffer: VecDeque::new() }
    }

    pub fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }

    pub fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
}

pub fn flush() {
    std::io::stdout().flush().unwrap();
}

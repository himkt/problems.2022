#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point(i64, i64);

impl std::ops::Add for Point {
    type Output=Self;
    fn add(self, _rhs: Point) -> Self { Self(self.0 + _rhs.0, self.1 + _rhs.1) }
}

impl std::ops::Sub for Point {
    type Output=Self;
    fn sub(self, _rhs: Point) -> Self { Self(self.0 - _rhs.0, self.1 - _rhs.1) }
}

impl std::ops::Mul<i64> for Point {
    type Output=Self;
    fn mul(self, k: i64) -> Self { Self(k * self.0, k * self.1) }
}

impl std::ops::Div<i64> for Point {
    type Output=Self;
    fn div(self, k: i64) -> Self { Self(self.0 / k, self.1 / k) }
}


pub struct Line(pub Point, pub Point);

impl Line {
    pub fn contains(&self, p: Point) -> bool {
        let d = self.1 - self.0;
        let e = p      - self.0;
        d.1 * e.0 == d.0 * e.1
    }
}


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let n: usize = scanner.cin();
    let k: usize = scanner.cin();

    let points: Vec<Point> = (0..n)
        .map(|_| {
            let x: i64 = scanner.cin();
            let y: i64 = scanner.cin();
            Point(x, y)
        })
        .collect();

    if k == 1 {
        println!("Infinity");
        return;
    }

    let mut pointsset: HashSet<Vec<usize>> = HashSet::new();

    for i in 0..n-1 {
        for j in i+1..n {
            let mut pointset: HashSet<usize> = HashSet::new();
            for k in 0..n {
                let reference = points[k];
                let line = Line(points[i], points[j]);

                if line.contains(reference) {
                    pointset.insert(k);
                }
            }
            let mut points: Vec<usize> = pointset
                .into_iter()
                .collect();

            points.sort_unstable();
            pointsset.insert(points);
        }
    }

    let mut ans = 0;
    for pointset in pointsset {
        if pointset.len() >= k {
            ans += 1;
        }
    }

    println!("{}", ans);
}


use std::collections::{VecDeque, HashSet};
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

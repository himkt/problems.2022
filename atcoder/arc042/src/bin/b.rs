use std::ops::{Add, Sub, Mul, Div};


// FIXME (himkt): Eq for Point<f64> won't work as expected.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point<T>(pub T, pub T);

impl<T: Add<T, Output=T>> std::ops::Add<Point<T>> for Point<T> {
    type Output = Point<T>;
    fn add(self, _rhs: Point<T>) -> Self::Output {
        Point(self.0 + _rhs.0, self.1 + _rhs.1)
    }
}

impl<T: Sub<T, Output=T>> std::ops::Sub<Point<T>> for Point<T> {
    type Output = Point<T>;
    fn sub(self, _rhs: Point<T>) -> Self {
        Point(self.0 - _rhs.0, self.1 - _rhs.1)
    }
}

impl<T: Copy + Mul<T, Output=T>> std::ops::Mul<T> for Point<T> {
    type Output = Point<T>;
    fn mul(self, k: T) -> Self {
        Self(k * self.0, k * self.1)
    }
}

impl<T: Copy + Div<T, Output=T>> std::ops::Div<T> for Point<T> {
    type Output = Self;
    fn div(self, k: T) -> Self {
        Self(self.0 / k, self.1 / k)
    }
}

impl<T: Mul<T, Output=T> + Sub<T, Output=T>> Point<T> {
    pub fn det(self, _rhs: Point<T>) -> T {
        self.0 * _rhs.1 - self.1 * _rhs.0
    }
}


pub struct Line<T>(pub Point<T>, pub Point<T>);


pub trait LineAPI<T> {
    fn distance(&self, p: Point<T>) -> f64;
    fn contains_point(&self, p: Point<T>) -> bool;
}

// i64
impl LineAPI<i64> for Line<i64> {
    fn distance(&self, p: Point<i64>) -> f64 {
        let v1 = self.1 - self.0;
        let v2 = p - self.0;
        let l1 = ((v1.0 * v1.0 + v1.1 * v1.1) as f64).sqrt();
        let det = v1.det(v2) as f64;
        det / l1
    }

    fn contains_point(&self, p: Point<i64>) -> bool {
        let d = self.1 - self.0;
        let e = p - self.0;
        d.1 * e.0 == d.0 * e.1
    }
}

// f64
impl LineAPI<f64> for Line<f64> {
    fn distance(&self, p: Point<f64>) -> f64 {
        let v1 = self.1 - self.0;
        let v2 = p - self.0;
        let l1 = (v1.0 * v1.0 + v1.1 * v1.1).sqrt();
        let det = v1.det(v2);
        det / l1
    }

    fn contains_point(&self, p: Point<f64>) -> bool {
        let d = self.1 - self.0;
        let e = p - self.0;
        d.1 * e.0 == d.0 * e.1
    }
}


const INF: f64 = std::f64::MAX;


#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let x: f64 = scanner.cin();
    let y: f64 = scanner.cin();
    let p = Point(x, y);

    let n: usize = scanner.cin();
    let mut points = vec![];
    for _ in 0..n {
        let xi: f64 = scanner.cin();
        let yi: f64 = scanner.cin();
        let pi = Point(xi, yi);
        points.push(pi);
    }

    let mut lines = vec![];
    for i in 0..n {
        let line = Line(points[i], points[(i + 1) % n]);
        lines.push(line);
    }

    let mut ans = INF;
    for i in 0..n {
        let d = lines[i].distance(p);
        ans = ans.min(d);
    }

    println!("{}", ans);
}


use std::collections::VecDeque;
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

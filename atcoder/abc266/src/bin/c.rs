use std::ops::{Add, Div, Mul, Sub};

// FIXME (himkt): Eq for Point<f64> won't work as expected.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Point<T>(pub T, pub T);

impl<T: Add<T, Output = T>> std::ops::Add<Point<T>> for Point<T> {
    type Output = Point<T>;
    fn add(self, _rhs: Point<T>) -> Self::Output {
        Point(self.0 + _rhs.0, self.1 + _rhs.1)
    }
}

impl<T: Sub<T, Output = T>> std::ops::Sub<Point<T>> for Point<T> {
    type Output = Point<T>;
    fn sub(self, _rhs: Point<T>) -> Self {
        Point(self.0 - _rhs.0, self.1 - _rhs.1)
    }
}

impl<T: Copy + Mul<T, Output = T>> std::ops::Mul<T> for Point<T> {
    type Output = Point<T>;
    fn mul(self, k: T) -> Self {
        Self(k * self.0, k * self.1)
    }
}

impl<T: Copy + Div<T, Output = T>> std::ops::Div<T> for Point<T> {
    type Output = Self;
    fn div(self, k: T) -> Self {
        Self(self.0 / k, self.1 / k)
    }
}

impl<T: Mul<T, Output = T> + Sub<T, Output = T>> Point<T> {
    pub fn det(self, _rhs: Point<T>) -> T {
        self.0 * _rhs.1 - self.1 * _rhs.0
    }
}


use std::collections::VecDeque;

pub fn convex_hull(ps: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let n = ps.len();

    let mut ps: Vec<Point<i64>> = ps.iter().map(|&(x, y)| Point::<i64>(x, y)).collect();

    ps.sort();

    let mut k = 0;
    let mut deque: VecDeque<Point<i64>> = VecDeque::new();

    for &pi in &ps {
        while k > 1 && (deque[k - 1] - deque[k - 2]).det(pi - deque[k - 1]) <= 0 {
            deque.pop_back();
            k -= 1;
        }
        deque.push_back(pi);
        k += 1;
    }

    let t = k;
    for i in (0..n - 1).rev() {
        let pi = ps[i];
        while k > t && (deque[k - 1] - deque[k - 2]).det(pi - deque[k - 1]) <= 0 {
            deque.pop_back();
            k -= 1;
        }
        deque.push_back(pi);
        k += 1;
    }

    let mut ret: Vec<(i64, i64)> = deque
        .into_iter()
        .take(k - 1)
        .map(|pair| (pair.0, pair.1))
        .collect();

    ret.sort_unstable();
    ret
}

#[allow(clippy::needless_range_loop)]
fn main() {
    let mut scanner = Scanner::new();
    let ax: i64 = scanner.cin();
    let ay: i64 = scanner.cin();
    let bx: i64 = scanner.cin();
    let by: i64 = scanner.cin();
    let cx: i64 = scanner.cin();
    let cy: i64 = scanner.cin();
    let dx: i64 = scanner.cin();
    let dy: i64 = scanner.cin();

    let p1 = (ax, ay);
    let p2 = (bx, by);
    let p3 = (cx, cy);
    let p4 = (dx, dy);
    let points = vec![p1, p2, p3, p4];

    let hull = convex_hull(points);
    if hull.len() == 4 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}

use std::io::Write;
pub struct Scanner {
    buffer: std::collections::VecDeque<String>,
    buf: String,
}
#[allow(clippy::new_without_default)]
impl Scanner {
    pub fn new() -> Self {
        Scanner {
            buffer: std::collections::VecDeque::new(),
            buf: String::new(),
        }
    }
    pub fn cin<T: std::str::FromStr>(&mut self) -> T {
        if !self.buffer.is_empty() {
            return self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap();
        }
        self.buf.truncate(0);
        std::io::stdin().read_line(&mut self.buf).ok();
        self.buf
            .to_owned()
            .split_whitespace()
            .for_each(|x| self.buffer.push_back(String::from(x)));
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
    pub fn flush(&self) {
        std::io::stdout().flush().unwrap();
    }
}

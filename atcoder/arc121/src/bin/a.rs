use std::cmp::Ordering;

use proconio::input;


#[derive(Copy,Clone,Eq,PartialEq,Debug)]
struct Point {
    p1: (i64, i64),
    p2: (i64, i64),
    dist: i64,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }

}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn solve(_: i64, points: Vec::<(i64, i64)>) {
    let mut queue = std::collections::BinaryHeap::new();
    for i in 0..points.len() {
        for j in i..points.len() {
            if i == j {
                continue
            }
            let dist = std::cmp::max((points[i].0 - points[j].0).abs(), (points[i].1 - points[j].1).abs());
            let p = Point { p1: points[i], p2: points[j], dist };
            queue.push(p);
        }
    }

    queue.pop();
    let point = queue.pop().unwrap();
    println!("{}", point.dist);
}


fn main() {
    input! {
        n: i64,
        points: [(i64, i64); n],
    }

    solve(n, points);
}

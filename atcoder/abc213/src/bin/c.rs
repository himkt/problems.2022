use std::collections::HashSet;

use proconio::input;


trait BinarySearch<T> {
    fn lower_bound(&self, f: impl Fn(&T) -> bool) -> usize;
}

impl<T> BinarySearch<T> for [T] {
    fn lower_bound(&self, f: impl Fn(&T) -> bool) -> usize {
        let mut left: isize = -1;
        let mut right = self.len() as isize;

        while right - left > 1 {
            let mid = (left + right) / 2;
            if f(&self[mid as usize]) {
                right = mid;
            } else {
                left = mid;
            }
        }

        right as usize
    }
}


fn main() {
    input! {
        _: usize,
        _: usize,
        n: usize,
        ab: [(usize, usize); n]
    }

    let aas: HashSet<usize> = ab.iter().map(|pair| pair.0).collect();
    let bbs: HashSet<usize> = ab.iter().map(|pair| pair.1).collect();

    let mut avs: Vec<usize> = aas.into_iter().collect();
    let mut bvs: Vec<usize> = bbs.into_iter().collect();

    avs.sort_unstable();
    bvs.sort_unstable();

    for (ai, bi) in ab.iter() {
        let aix = avs.lower_bound(|k| k > ai);
        let bix = bvs.lower_bound(|k| k > bi);
        println!("{} {}", aix, bix);
    }
}

use proconio::input;
use std::collections::HashSet;


// Replicate the existing submission line-by-line
// ref. https://atcoder.jp/contests/arc121/submissions/22996682
fn main() {
    input! {
        n: usize,
        p: [(i64, i64); n]
    }

    let mut ppp = vec![];
    for (i, pi) in p.iter().enumerate() {
        ppp.push((pi.0, pi.1, i));
    }

    // [memo] max(|xi-xj|, |yi-yj|) can be decomposed to
    //        max(max(|xi-xj), max(|yi-yj|)) since each element of
    //        arguments passed to max is independent.
    let mut xp = ppp.clone();
    xp.sort_by(|p1, p2| p1.0.cmp(&p2.0));

    let mut yp = ppp;
    yp.sort_by(|p1, p2| p1.1.cmp(&p2.1));

    // [memo] it uses HashSet to overwrite same point pair?
    let mut pp = HashSet::new();
    for i in 0..=(2.min(xp.len())) {
        pp.insert(&xp[i]);
        pp.insert(&yp[i]);
        pp.insert(&xp[xp.len() - 1 - i]);
        pp.insert(&yp[yp.len() - 1 - i]);
    }

    // [memo] O(N^2) distance calculation for the small set of points
    let pv: Vec<&(i64, i64, usize)> = pp.into_iter().collect();
    let mut ll = vec![];
    for i in 0..(pv.len() - 1) {
        for j in (i + 1)..pv.len() {
            let l = (pv[i].0 - pv[j].0).abs().max((pv[i].1 - pv[j].1).abs());
            ll.push(l);
        }
    }

    ll.sort_unstable();
    println!("{}", ll[ll.len() - 2]);
}

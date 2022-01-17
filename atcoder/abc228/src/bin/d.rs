use std::collections::BTreeSet;

use proconio::input;


const N: usize = 1 << 20;


fn solve(_: usize, tx: Vec::<(usize, usize)>) {
    let mut set: BTreeSet<usize> = (0..N).into_iter().collect();
    let mut vec = vec![-1i64; N];

    for (t, x) in tx.into_iter() {
        let key = x % N;

        if t == 2 {
            println!("{}", vec[key]);
        }
        else {
            let i = if let Some(&i) = set.range(key..).next() {
                i
            } else {
                *set.range(..).next().unwrap()
            };

            vec[i] = x as i64;
            set.remove(&i);
        }
    }
}


fn main() {
    input! {
        q: usize,
        tx: [(usize, usize); q],
    }

    solve(q, tx);
}

use std::collections::HashSet;

use proconio::input;


fn solve() {
}


fn main() {
    input! {
        x: i32,
        y: i32,
    }

    if x == y {
        println!("{}", x);
    }
    else {
        let mut hsh = HashSet::new();
        hsh.insert(x);
        hsh.insert(y);

        for k in 0..3 {
            if !hsh.contains(&k) {
                println!("{}", k);
                return
            }
        }
    }
    solve();
}

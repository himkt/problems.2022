use std::collections::HashSet;

use proconio::input;


fn is_weak(x: String) -> bool {
    let s: HashSet<char> = x.chars().into_iter().collect();
    if s.len() == 1 {
        return true
    }

    let mut xprev = x.chars().next().unwrap().to_digit(10).unwrap();
    for xi in x[1..].chars() {
        let xi = xi.to_digit(10).unwrap();

        if xprev >= xi && !(xprev == 9 && xi == 0) {
            return false;
        }

        if xprev + 1 == xi || (xprev == 9 && xi == 0) {
            xprev = xi;
            continue
        }
        else {
            return false;
        }
    }

    true
}


fn main() {
    input! {
        x: String,
    }

    if is_weak(x) {
        println!("Weak");
    }
    else {
        println!("Strong");
    }
}

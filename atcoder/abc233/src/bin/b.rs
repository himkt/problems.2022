use proconio::input;
use std::iter::FromIterator;


fn main() {
    input! {
        l: usize,
        r: usize,
        s: String,
    }

    let mut cs: Vec<char> = s.chars().collect();
    cs[l-1..r].reverse();

    println!("{}", String::from_iter(cs));
}

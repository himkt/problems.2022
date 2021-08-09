use proconio::input;


fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let mut ans = 0;
    for k in 0..=255 {
        if a ^ k == b {
            ans = k;
        }
    }

    println!("{}", ans);
}

use proconio::input;


fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut ans = 0;
    for ai in a.into_iter() {
        if ai > 10 {
            ans += ai - 10;
        }
    }

    println!("{}", ans);
}

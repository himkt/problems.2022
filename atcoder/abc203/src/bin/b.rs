use proconio::input;


fn main() {
    input! {
        n: i32,
        k: i32,
    }

    let mut ans = 0;
    for ni in 0..n {
        for ki in 0..k {
            ans += 100*(ni+1) + (ki+1);
        }
    }

    println!("{}", ans);
}

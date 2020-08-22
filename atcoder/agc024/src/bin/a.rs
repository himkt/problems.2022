use proconio::input;


fn solve(a: i128, b: i128, _: i128, k: i128) {
    if k % 2 == 0 {
        println!("{}", a-b);
    }
    else {
        println!("{}", b-a);
    }
}


fn main() {
    input! {
        a: i128,
        b: i128,
        c: i128,
        k: i128,
    }

    solve(a, b, c, k);
}

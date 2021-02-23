use proconio::input;


fn solve (n: i32, _: i32, a: Vec<i32>) {
    let sum_a: i32 = a.iter().sum();
    let ans =
        if n >= sum_a { n - sum_a }
        else { -1 };

    println!("{}", ans);
}


fn main() {
    input! {
       n: i32,
       m: i32,
       a: [i32; m],
    }
    solve(n, m, a);
}

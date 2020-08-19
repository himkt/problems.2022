use proconio::input;


fn solve(_: i32, x: i32, l: Vec::<i32>) {
    let mut ans: i32 = 1;
    let mut cur: i32 = 0;

    for li in l {
        if cur + li <= x {
            ans += 1;
        }
        cur += li;
    }

    println!("{}", ans);
}


fn main() {
    input! {
        n: i32,
        x: i32,
        l: [i32; n],
    }

    solve(n, x, l);
}

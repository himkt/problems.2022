use proconio::input;


fn solve(n: i64, mut x: i64, a: Vec::<i64>) -> i64 {
    let mut ans = 0;
    for i in 0..n as usize {
        x -= a[i];
        ans += 1;

        if x <= 0 {
            break;
        }
    }

    if x != 0 {
        ans -= 1;
    }

    return ans;
}


fn main() {
    input!{
        n: i64,
        mut x: i64,
        mut a: [i64; n],
    }

    a.sort();
    println!("{}", solve(n, x, a));
}

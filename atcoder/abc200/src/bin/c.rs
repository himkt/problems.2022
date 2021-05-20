use proconio::input;


fn combination(n: i64, mut r: i64) -> i64 {
    if r > n {
        return 0;
    }

    if r * 2 > n {
        r = n - r;
    }

    if r == 0 {
        return 1;
    }

    let mut res = 1;
    for i in 1..(r+1) {
        res *= n - i + 1;
        res /= i;
    }

    res
}


fn solve(_: i64, a: Vec<i64>) {
    let mut b = Vec::<i64>::new();
    b.resize(200, 0);

    for ai in a {
        b[(ai % 200) as usize] += 1;
    }

    let mut ans = 0;
    for bi in b {
        ans += combination(bi, 2);
    }

    println!("{}", ans);
}


fn main() {
    input! {
        n: i64,
        a: [i64; n],
    }
    solve(n, a);
}

use proconio::input;


fn solve(mut n: i64, k: i64) {
    for _ in 0..k {
        if n % 200 == 0 {
            n /= 200;
        }
        else {
            n *= 1000;
            n += 200;
        }
    }

    print!("{}", n);
}

fn main() {
    input! {
        n: i64,
        k: i64,
    }
    solve(n, k);
}

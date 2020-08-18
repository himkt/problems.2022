use proconio::input;


fn solve(mut h: i64) -> i64 {
    let mut i = 1;

    loop {
        if h > 1 {
            h /= 2;
            i += 1;
        }
        else {
            break;
        }
    }

    return 2i64.pow(i) - 1;
}


fn main() {
    input! {
        h: i64,
    }

    println!("{}", solve(h));
}

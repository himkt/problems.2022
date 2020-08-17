use proconio::input;


fn factorize(mut base: i64) -> Vec<(i64, i64)> {
    let mut fs = Vec::<(i64, i64)>::new();
    let mut i: i64 = 2;

    while i*i <= base {
        if base % i == 0 {
            let mut cnt: i64 = 0;

            while base % i == 0 {
                base /= i;
                cnt += 1;
            }
            fs.push((i, cnt));
        }
        i += 1;
    }

    if base > 1 {
        fs.push((base, 1));
    }

    return fs;
}


fn main() {
    input! {
        n: i32,
        a: [i64; n],
    }

    let mut ans = 0;

    for ai in a {
        for (i, c) in factorize(ai) {
            if i == 2 {
                ans += c;
            }
        }
    }

    println!("{}", ans);
}

use proconio::input;


fn points(pairs: &[(i64, i64)], xl: i64, xr: i64, yu: i64, yd: i64) -> usize {
    let xmin = xl.min(xr);
    let xmax = xl.max(xr);
    let ymin = yu.min(yd);
    let ymax = yu.max(yd);

    let mut ans = 0;
    for &(x, y) in pairs {
        if !(xmin <= x && x <= xmax) {
            continue;
        }
        if !(ymin <= y && y <= ymax) {
            continue;
        }
        ans += 1;
    }
    ans
}


fn s(xl: i64, xr: i64, yu: i64, yd: i64) -> i64 {
    let xmin = xl.min(xr);
    let xmax = xl.max(xr);
    let ymin = yu.min(yd);
    let ymax = yu.max(yd);

    (xmax - xmin) * (ymax - ymin)
}

fn main() {
    input! {
        n: usize,
        k: usize,
        pairs: [(i64, i64); n],
    }

    let xs: Vec<i64> = (&pairs).iter().map(|&p| p.0).collect();
    let ys: Vec<i64> = (&pairs).iter().map(|p| p.1).collect();

    // S_{max} = 2*10^9 x 2*10^9
    let mut ans: i64 = 4_000_000_000_000_000_000;

    for a in 0..n {
        for b in a+1..n {
            for c in 0..n {
                for d in c+1..n {
                    let xl = xs[a];
                    let xr = xs[b];
                    let yu = ys[c];
                    let yd = ys[d];
                    if points(&pairs, xl, xr, yu, yd) == k {
                        ans = ans.min(s(xl, xr, yu, yd));
                    }
                }
            }
        }
    }

    println!("{}", ans);
}

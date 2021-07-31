use proconio::input;


fn solve() {
}


fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [i64; n],
        mut b: [i64; m],
    }

    a.sort_unstable();
    b.sort_unstable();

    let mut i = 0;
    let mut j = 0;

    let mut ans: i64 = (a[i] - b[j]).abs();

    loop {
        if (i == a.len() - 1) && (j == b.len() - 1) {
            break;
        }

        if a[i] == b[j] {
            ans = 0;
            break;
        }

        if a[i] > b[j] {
            if j == b.len() - 1 {
                i += 1;
            }
            else {
                j += 1;
            }
        }
        else {
            if i == a.len() - 1 {
                j += 1;
            }
            else {
                i += 1;
            }
        }

        ans = (a[i] - b[j]).abs().min(ans);
    }

    println!("{}", ans);
    solve();
}

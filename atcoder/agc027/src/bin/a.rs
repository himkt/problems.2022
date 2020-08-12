use proconio::input;

fn main() {
    input!{
        n: i64,
        x: i64,
        mut a: [i64; n],
    }

    let mut x = x;
    let mut ans = 0;

    a.sort();

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

    println!("{}", ans);
}

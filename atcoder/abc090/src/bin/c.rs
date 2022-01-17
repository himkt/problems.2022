use proconio::input;


fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut ans = 0;
    if n == 1 && m == 1 {
        ans = 1;
    }
    else if n == 1 && m > 1 {
        ans = m - 2;
    }
    else if n > 1 && m == 1 {
        ans = n - 2;
    }
    else if n > 2 && m > 2 {
        ans = (n-2) * (m-2);
    }

    println!("{}", ans);
}

use proconio::input;


fn solve(n: i32, a: Vec::<usize>, b: Vec::<i32>, c: Vec<i32>) {
    let mut ans: i32 = 0;

    for i in 0..(n-1) as usize {
        if a[i]+1 == a[i+1] {
            ans += c[a[i]-1];
        }
    }

    for bi in b {
        ans += bi;
    }

    println!("{}", ans);
}

fn main() {
    input! {
        n: i32,
        a: [usize; n],
        b: [i32; n],
        c: [i32; n-1],
    }
    solve(n, a, b, c);
}

use proconio::input;


fn factorial(k: i32) -> i32 {
    let mut res = 1;
    for k in 1..=k {
        res *= k;
    }
    res
}


fn main() {
    input! {
        mut p: i32,
    }

    let mut coins = vec![];
    for k in 1..=10 {
        coins.push(factorial(k));
    }
    coins.reverse();

    let mut cur = 0;
    let mut ans = 0;
    while p!= 0 {
        if coins[cur] <= p {
            p -= coins[cur];
            ans += 1;
        }
        else {
            cur += 1;
        }
    }

    println!("{}", ans);
}

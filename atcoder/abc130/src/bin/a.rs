use proconio::input;


fn solve(x: i32, a: i32) {
    if x < a {
        println!("0");
    }
    else {
        println!("10");
    }
}


fn main() {
    input! {
        x: i32,
        a: i32,
    }

    solve(x, a);
}

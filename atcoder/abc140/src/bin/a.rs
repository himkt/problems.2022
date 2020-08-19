use proconio;

fn solve(n: i32) {
    println!("{}", n.pow(3));
}

fn main() {
    proconio::input! {
        n: i32,
    }
    solve(n);
}

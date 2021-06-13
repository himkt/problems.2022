use proconio::input;


fn main() {
    input! {
        a: f32,
        b: f32,
    }

    println!("{}", a * b / 100.0);
}

use proconio::input;


fn main() {
    input! {
        s: String,
    }

    let l = s.len();
    let a = s.chars().nth(0).unwrap();
    let b = String::from(&s[1..l]);
    println!("{}", format!("{}{}", b, a));
}

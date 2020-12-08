use proconio::input;


fn solve (s: Vec<char>, k: i128) {
    if k == 1 {
        println!("{}", s[0]);
        return
    }

    let mut i = 1;
    for c in s {
        if c != '1' || i == k {
            println!("{}", c);
            return
        }
        i += 1;
    }

    println!("1");
}


fn main() {
    input! {
        s: String,
        k: i128,
    }

    solve(s.chars().collect(), k);
}

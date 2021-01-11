use proconio::input;


fn solve (s: String) {
    if s == "keyence" {
        println!("YES");
        return;
    }

    for i in 0..s.len()+1 {
        for j in i+1..s.len()+1 {
            if "keyence" == (&s[0..i]).to_string() + &s[j..s.len()] {
                println!("YES");
                return
            }
        }
    }

    println!("NO");
}


fn main() {
    input! {
        s: String,
    }

    solve(s);
}

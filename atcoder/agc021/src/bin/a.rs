use proconio::input;


fn solve (n: i64) {
    let s = n.to_string();

    let uniq: std::collections::HashSet<i32> = s
        .chars()
        .map(|c| c as i32 - '0' as i32)
        .collect();

    let head_num = s.chars().next().unwrap() as i32 - '0' as i32;
    let spec: std::collections::HashSet<i32> = [9, head_num]
        .iter()
        .copied()
        .collect();

    if s.len() == 1 {
        println!("{}", head_num);
    }
    else if uniq == spec {
        println!("{}", head_num + (s.len() as i32 - 1) * 9);
    }
    else {
        println!("{}", (head_num - 1) + (s.len() as i32 - 1) * 9);
    }
}


fn main() {
    input! {
        n: i64,
    }
    solve(n);
}

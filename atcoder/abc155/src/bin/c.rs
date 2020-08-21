use proconio::input;


fn solve(s: Vec::<String>) {
    let mut c = std::collections::HashMap::new();

    for si in &s {
        *c.entry(si).or_insert(1) += 1;
    }

    let max = c.values().max().unwrap();
    let mut ns = Vec::new();

    for (k, v) in &c {
        if v == max {
            ns.push(k);
        }
    }

    ns.sort();

    for si in ns {
        println!("{}", si);
    }
}


fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    solve(s);
}

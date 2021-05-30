use proconio::input;


fn main() {
    input! {
        n: i64,
        k: i64,
        mut ab: [(i64, i64); n],
    }

    ab.sort_by(|a, b| a.0.cmp(&b.0));
    let a: Vec<i64> = ab.iter().map(|x| x.0).collect();
    let b: Vec<i64> = ab.iter().map(|x| x.1).collect();

    let mut money: i64 = k;

    for (ai, bi) in a.into_iter().zip(b) {
        if money >= ai {
            money += bi;
        }
        else {
            break;
        }
    }

    println!("{}", money);
}

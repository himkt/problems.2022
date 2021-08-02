use proconio::input;


fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i64; n],
    }

    let mut dist = vec![1001001001; n];
    dist[0] = 0;

    for i in 0..n {
        for diff in 1..=k {
            let j = i + diff;

            if j >= n {
                continue;
            }

            dist[j] = dist[j].min(dist[i] + (h[i]-h[j]).abs());
        }
    }

    println!("{}", dist.last().unwrap());
}

use proconio::input;


fn solve (q: i128, h: i128, s: i128, d: i128, mut n: i128) {
    let prices = vec![q, h, s, d];
    let litters: Vec<f64> = vec![0.25, 0.5, 1.0, 2.0];
    let weights: Vec<f64> = vec![q as f64 * 4.0, h as f64 * 2.0, s as f64, d as f64 * 0.5];

    let mut indices: Vec<usize> = vec![0, 1, 2, 3];
    indices.sort_by(|&a, &b| (&weights[a]).partial_cmp(&weights[b]).unwrap());

    let mut num_teas_by_idx: Vec<i128> = vec![0; 4];
    let mut num_teas;
    for idx in indices {
        if n as f64 >= litters[idx] && litters[idx] <= n as f64 {
            num_teas = (n as f64 / litters[idx]).floor();
            num_teas_by_idx[idx] = num_teas as i128;
            n -= (litters[idx] * num_teas) as i128;
        }
    }

    let ans: i128 = [0, 1, 2, 3].iter().map(|&idx|num_teas_by_idx[idx] * prices[idx]).sum();
    println!("{}", ans);
}


fn main() {
    input! {
        q: i128,
        h: i128,
        s: i128,
        d: i128,
        n: i128,
    }

    solve(q, h, s, d, n);
}

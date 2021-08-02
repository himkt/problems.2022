use proconio::input;


fn main() {
    input! {
        n: usize,
        happinesses: [[usize; 3]; n],
    }

    let mut dist = vec![vec![0; 3]; n];
    for i in 0..3 {
        dist[0][i] = happinesses[0][i];
    }

    for t in 1..n {
        for k in 0..3 {
            let p= dist[t-1]
                .iter()
                .enumerate()
                .filter(|(idx, _)| *idx != k)
                .map(|(_, v)| *v)
                .max()
                .unwrap();

            dist[t][k] = happinesses[t][k] + p;
        }
    }

    println!("{}", dist[n-1].iter().max().unwrap());
}

use proconio::input;


const INF: usize = 1001001001;


#[allow(clippy::needless_range_loop)]
fn main () {
    input! {
        n: usize,
        m: usize,
        paths: [(usize, usize, usize); m],
    }

    let mut graph = vec![vec![INF; n]; n];
    for i in 0..n {
        graph[i][i] = 0;
    }

    for (s, t, c) in paths {
        graph[s-1][t-1] = c;
    }

    let mut ans = 0;
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                graph[i][j] = graph[i][j].min(graph[i][k] + graph[k][j]);
                if graph[i][j] != INF {
                    ans += graph[i][j];
                }
            }
        }
    }

    println!("{}", ans);
}

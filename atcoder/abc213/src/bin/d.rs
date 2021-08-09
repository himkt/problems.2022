#![allow(clippy::needless_range_loop)]
use proconio::input;


const INF: usize = 1001001001;


// This implementation doesn't have `seen`,
// because it runs on a tree, which has the exactly one parent for each node.
#[derive(Debug,Clone)]
pub struct DFS {
    graph: Vec<Vec<usize>>,
    seen: Vec<bool>,
    visits: Vec<usize>,
}

impl DFS {
    pub fn new(graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        Self {
            graph,
            seen: vec![false; n],
            visits: vec![],
        }
    }

    pub fn search(&mut self, root: usize, parent: usize) {
        self.dfs(root, parent);
    }

    pub fn dfs(&mut self, v: usize, parent: usize) {
        self.visits.push(v+1);

        for i in 0..self.graph[v].len() {
            if self.graph[v][i] == parent {
                continue;
            }
            self.dfs(self.graph[v][i], v);
            self.visits.push(v+1);
        }
    }
}


fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n-1],
    }

    let mut graph = vec![Vec::new(); n];
    for (a, b) in ab.into_iter() {
        graph[a-1].push(b-1);
        graph[b-1].push(a-1);
    }

    for i in 0..n {
        graph[i].sort_unstable();
    }

    let mut dfs = DFS::new(graph);
    dfs.search(0, INF);
    println!("{}", dfs.visits.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
}

use proconio::input;


#[derive(Debug,Clone)]
pub struct DFS {
    graph: Vec<Vec<usize>>,
    seen: Vec<bool>,
    num_childrens: Vec<usize>,
}

impl DFS {
    pub fn new(graph: Vec<Vec<usize>>) -> Self {
        let n = graph.len();
        Self {
            graph,
            seen: vec![false; n],
            num_childrens: vec![0; n],
        }
    }

    pub fn search(&mut self, root: usize) {
        self.dfs(root, 0);
    }

    pub fn dfs(&mut self, v: usize, dist: usize) {
        if self.seen[v] {
            return;
        }

        self.seen[v] = true;
        for i in 0..self.graph[v].len() {
            self.dfs(self.graph[v][i], dist+1);
        }
        self.num_childrens[v] = self.graph[v].len();
    }
}


fn main () {
    input! {
        n: usize,
        a: [usize; n-1],
    }

    let mut g = vec![vec![]; n];
    for (i, ai) in a.into_iter().enumerate() {
        g[ai-1].push(i+1);
    }

    let mut searcher = DFS::new(g);
    searcher.search(0);

    for num_children in searcher.num_childrens.into_iter() {
        println!("{}", num_children);
    }
}

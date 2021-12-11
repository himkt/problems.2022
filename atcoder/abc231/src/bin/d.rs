use proconio::input;


#[derive(Debug,Clone)]
pub struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>
}


#[allow(clippy::needless_range_loop)]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            ranks: vec![1usize; n]
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parents[x] == x {
            x
        }
        else {
            self.parents[x] = self.find(self.parents[x]);
            self.parents[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let mut px = self.find(x);
        let mut py = self.find(y);

        if px == py {
            return;
        }

        if self.ranks[px] > self.ranks[py] {
            std::mem::swap(&mut px, &mut py);
        }

        if self.ranks[px] == self.ranks[py] {
            self.ranks[py] += 1;
        }

        self.parents[px] = py;
    }
}


fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }

    let mut g = vec![vec![]; n];
    for (source, target) in edges.iter() {
        g[source-1].push(target-1);
        g[target-1].push(source-1);
    }

    let num_edges_many = g
        .iter()
        .filter(|edges| edges.len() > 2)
        .count();

    if num_edges_many > 0 {
        println!("No");
        return;
    }
    else {
        let mut uf = UnionFind::new(n);
        for (x, y) in edges.into_iter() {
            if uf.find(x-1) == uf.find(y-1) {
                println!("No");
                return;
            }
            uf.unite(x-1, y-1);
        }
    }

    println!("Yes");
}

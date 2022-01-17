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
        p: [usize; n],
        edges: [(usize, usize); m],
    }

    let mut uf = UnionFind::new(n);
    for (x, y) in edges {
        uf.unite(x-1, y-1);
    }

    let mut ans = 0;
    for (i, &pi) in p.iter().enumerate() {
        if uf.find(pi-1) == uf.find(i) { ans += 1}
    }

    println!("{}", ans);
}

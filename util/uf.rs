struct UnionFind {
    n: usize,
    par: Vec<usize>,
    cnt: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            n,
            par: Vec::from_iter(0..n),
            cnt: vec![1; n],
        }
    }
    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            self.par[x] = self.root(self.par[x]);
            self.par[x]
        }
    }
    #[allow(dead_code)]
    fn root_imt(&self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            self.root_imt(self.par[x])
        }
    }
    #[allow(dead_code)]
    fn group_size(&mut self, i: usize) -> usize {
        let ri = self.root(i);
        self.cnt[ri]
    }
    fn unite(&mut self, i: usize, j: usize) {
        let ri = self.root(i);
        let rj = self.root(j);
        if ri == rj {
            return;
        }
        self.par[rj] = ri;
        self.cnt[ri] += self.cnt[rj]
    }
    #[allow(dead_code)]
    fn same_group(&mut self, i: usize, j: usize) -> bool {
        self.root(i) == self.root(j)
    }
    #[allow(dead_code)]
    fn print(&mut self) {
        let mut cld = vec![vec![]; self.n];
        for i in 0..self.n {
            let ri = self.par[i];
            cld[ri].push(i);
        }
        println!("{:?}", cld);
        fn dfs(i: usize, indent: usize, cld: &Vec<Vec<usize>>, check: &mut Vec<bool>) {
            println!("{}", format!("\x1b[31m{i}\x1b[0m"));
            check[i] = true;
            for c in &cld[i] {
                if check[*c] {
                    continue;
                }
                for _ in 0..indent + 1 {
                    print!("--")
                }
                dfs(*c, indent + 1, cld, check);
            }
        }
        let mut check = vec![false; self.n];
        for (i, ci) in cld.iter().enumerate() {
            if ci.len() > 0 {
                dfs(i, 0, &cld, &mut check)
            }
        }
    }
}

#[allow(dead_code)]
fn kruskal(n_nd: usize, edges: &Vec<((usize, usize), usize)>) -> Vec<((usize, usize), usize)> {
    let mut res = vec![];
    let mut uf = UnionFind::new(n_nd);
    let mut edges = edges.clone();
    edges.sort_by(|a, b| a.1.cmp(&b.1));
    for ((e1, e2), cost) in edges {
        if !uf.same_group(e1, e2) {
            uf.unite(e1, e2);
            res.push(((e1, e2), cost));
        }
    }
    res
}

pub struct UnionFind {
    pub parent: Vec<usize>,
    pub size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        return UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        };
    }
    // xの親
    pub fn root(&self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }

        return self.root(self.parent[x]);
    }

    // uとvの結合
    // 値を変更するのでmutとしてselfを参照
    pub fn unite(&mut self, u: usize, v: usize) -> bool {
        if self.root(u) == self.root(v) {
            return false;
        }
        // union by size
        // ノード数が大きい方に，ノード数が小さい方を結合する
        if self.size[u] < self.size[v] {
            self.parent[u] = v;
            self.size[v] += self.size[u];
        } else {
            self.parent[v] = u;
            self.size[u] += self.size[v];
        }

        return true;
    }

    pub fn same(&self, u: usize, v: usize) -> bool {
        return self.root(u) == self.root(v);
    }
}

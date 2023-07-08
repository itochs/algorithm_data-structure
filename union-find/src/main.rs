pub struct UnionFind {
    pub parent: Vec<usize>,
    pub size: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        return UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        };
    }
    // xの親
    fn root(&self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }

        return self.root(self.parent[x]);
    }

    // uとvの結合
    // 値を変更するのでmutとしてselfを参照
    fn unite(&mut self, u: usize, v: usize) -> bool {
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

    fn same(&self, u: usize, v: usize) -> bool {
        return self.root(u) == self.root(v);
    }
}

fn main() {
    println!("Hello, world!");

    // {{0,1,2,3}, {4,5}, {6}}
    let mut uf = UnionFind::new(7);
    uf.unite(0, 1);
    uf.unite(0, 2);
    uf.unite(2, 3);
    uf.unite(4, 5);
    for i in 0..7 {
        println!("{} root: {}, parent: {}", i, uf.root(i), uf.parent[i]);
    }
    println!("1 0 is same?: {}", uf.same(1, 0));
    println!("5 6 is same?: {}", uf.same(5, 6));
}

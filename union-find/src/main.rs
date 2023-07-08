pub struct UnionFind {
    pub parent: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        return UnionFind {
            parent: (0..n).collect(),
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
        let ru = self.root(u);
        let rv = self.root(v);
        if ru == rv {
            return false;
        }

        self.parent[rv] = ru;

        return true;
    }

    fn same(&self, u: usize, v: usize) -> bool {
        return self.root(u) == self.root(v);
    }
}

fn main() {
    println!("Hello, world!");

    // {{0,1,2}, {3,4}, {5}, {6}}
    let mut uf = UnionFind::new(7);
    uf.unite(0, 1);
    uf.unite(0, 2);
    uf.unite(4, 3);
    for i in 0..7 {
        println!("{} root: {}", i, uf.root(i));
    }
    println!("1 0 is same?: {}", uf.same(1, 0));
    println!("5 6 is same?: {}", uf.same(5, 6));
}

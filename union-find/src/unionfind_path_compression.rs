pub struct UnionFindPathCompression {
    pub parent: Vec<usize>,
    pub size: Vec<usize>,
}
// UnionFind上でDPをやるとき経路圧縮をしたくないらしい
impl UnionFindPathCompression {
    pub fn new(n: usize) -> Self {
        return UnionFindPathCompression {
            parent: (0..n).collect(),
            size: vec![1; n],
        };
    }
    // xの親

    pub fn root(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }
        self.parent[x] = self.root(self.parent[x]);

        return self.root(self.parent[x]);
    }

    // uとvの結合
    // 値を変更するのでmutとしてselfを参照
    pub fn unite(&mut self, u: usize, v: usize) -> bool {
        let ru = self.root(u);
        let rv = self.root(v);
        if ru == rv {
            return false;
        }
        // union by size
        // ノード数が大きい方に，ノード数が小さい方を結合する
        dbg!(self.size[ru] < self.size[rv]);
        if self.size[ru] < self.size[rv] {
            self.parent[ru] = v;
            self.size[rv] += self.size[ru];
        } else {
            self.parent[rv] = ru;
            self.size[ru] += self.size[rv];
        }

        return true;
    }

    pub fn same(&mut self, u: usize, v: usize) -> bool {
        return self.root(u) == self.root(v);
    }
}

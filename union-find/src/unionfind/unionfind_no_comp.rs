use crate::unionfind::UnionFindTrait;
pub struct UnionFind {
    pub parent: Vec<usize>,
    pub size: Vec<usize>,
}
impl UnionFindTrait for UnionFind {
    fn new(n: usize) -> Self {
        return UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        };
    }
    fn root(&self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }

        return self.root(self.parent[x]);
    }
    fn unite(&mut self, u: usize, v: usize) -> bool {
        let ru = self.root(u);
        let rv = self.root(v);
        if ru == rv {
            return false;
        }

        if self.size[ru] < self.size[rv] {
            self.parent[ru] = rv;
            self.size[rv] += self.size[ru];
        } else {
            self.parent[rv] = ru;
            self.size[ru] += self.size[rv];
        }

        return true;
    }
    fn len(&self) -> usize {
        return (0..self.parent.len())
            .filter(|&i| i == self.parent[i])
            .collect::<Vec<usize>>()
            .len();
    }
}

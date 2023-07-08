pub mod unionfind_no_comp;
pub mod unionfind_path_compression;
pub trait UnionFindTrait {
    fn new(n: usize) -> Self;

    fn len(&self) -> usize;
    // xの親
    fn root(&self, x: usize) -> usize;

    // uとvの結合
    // 値を変更するのでmutとしてselfを参照
    fn unite(&mut self, u: usize, v: usize) -> bool;

    fn same(&self, u: usize, v: usize) -> bool {
        return self.root(u) == self.root(v);
    }
}

mod unionfind;
use unionfind::UnionFindTrait;
use unionfind::{unionfind_no_comp, unionfind_path_compression};

fn main() {
    println!("Hello, world!");
    // {{0,1,2,3}, {4,5}, {6}}
    let mut uf = unionfind_no_comp::UnionFind::new(7);
    uf.unite(0, 1);
    uf.unite(0, 2);
    uf.unite(2, 3);
    uf.unite(4, 5);
    for i in 0..7 {
        println!("{} root: {}, parent: {}", i, uf.root(i), uf.parent[i]);
    }
    println!("1 0 is same?: {}", uf.same(1, 0));
    println!("5 6 is same?: {}", uf.same(5, 6));

    let mut uf = unionfind_path_compression::UnionFind::new(7);
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

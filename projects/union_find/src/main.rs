//use std::collections::BTreeMap;

struct UnionFind{
    parent: Vec<usize>,
    conjugacy_classes: usize,
}

impl UnionFind{
    fn from(&n: &usize) -> UnionFind{
        let mut parent = Vec::<usize>::new();
        for i in 0..n{
            parent[i] = i;
        }
        UnionFind{
            parent,
            conjugacy_classes: n,
        }
    }
    fn union(&mut self, &i: &usize, &j:&usize){
        if self.parent[i] != self.parent[j]{
            self.parent[i] = self.parent[j];
        }
    }
    fn find(&self, &i: &usize) -> usize{
        let mut i = i;
        while self.parent[i] != i{
            i = self.parent[i];
        }
        i
    }
}

fn main() {
    println!("Hello, world!");
}

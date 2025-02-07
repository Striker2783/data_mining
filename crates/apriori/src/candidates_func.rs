use crate::hash_tree::AprioriHashTree;

pub fn join_tree<T: FnMut(&[usize]) -> bool>(v: &[&Vec<usize>], mut prune_fn: T) -> AprioriHashTree<50> {
    let mut tree = AprioriHashTree::<50>::default();
    for i in 0..v.len() {
        for j in (i+1)..v.len() {
            let c1 = v[i];
            let c2 = v[j];
            if c1[..(c1.len() - 1)] != c2[..(c1.len() - 1)] {
                continue;
            }
            let join = if c1.last().unwrap() > c2.last().unwrap() {
                let mut temp = c2.clone();
                temp.push(*c1.last().unwrap());
                temp
            } else {
                let mut temp = c1.clone();
                temp.push(*c2.last().unwrap());
                temp
            };
            if prune_fn(&join) {
                continue;
            }
            tree.add(&join);
        }
    }
    tree
}
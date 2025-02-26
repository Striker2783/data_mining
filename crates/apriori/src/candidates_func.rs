use std::collections::HashMap;

use crate::hash_tree::AprioriHashTree;

pub fn join_tree<T: FnMut(&[usize]) -> bool>(
    v: &[&Vec<usize>],
    mut prune_fn: T,
) -> AprioriHashTree {
    let mut tree = AprioriHashTree::default();
    join(v, |join| {
        if prune_fn(&join) {
            return;
        }
        tree.add(&join);
    });
    tree
}

pub fn join<T: FnMut(Vec<usize>)>(v: &[&Vec<usize>], mut f: T) {
    let mut map = HashMap::new();
    for c in v.iter() {
        map.entry(&c[..(c.len() - 1)])
            .and_modify(|v: &mut Vec<usize>| v.push(*c.last().unwrap()))
            .or_insert(vec![*c.last().unwrap()]);
    }
    for (k, v) in map.into_iter() {
        for i in 0..v.len() {
            for j in (i + 1)..v.len() {
                let c1 = v[i];
                let c2 = v[j];
                let mut join = k.to_vec();
                if c2 > c1 {
                    join.push(c1);
                    join.push(c2);
                } else {
                    join.push(c2);
                    join.push(c1);
                }
                f(join);
            }
        }
    }
}

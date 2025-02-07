use std::{collections::HashSet, hash::RandomState};

use crate::{apriori::Apriori, array2d::Array2D, hash_tree::AprioriHashTree};


#[derive(Debug, Default)]
pub struct Candidates {
    candidates: Vec<HashSet<Vec<usize>>>,
}

impl Candidates {
    pub fn new() -> Self {
        Candidates::default()
    }
    pub fn run(&mut self, data: &Apriori) {
        self.run_one(data);
        self.run_two(data);
        while !self.candidates.last().unwrap().is_empty() {
            let c_prev: Vec<_> = self.candidates.last().unwrap().iter().collect();
            let mut tree = AprioriHashTree::<50>::default();
            for i in 0..c_prev.len() {
                for j in 0..i {
                    let c1 = c_prev[i];
                    let c2 = c_prev[j];
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
                    if self.can_be_pruned(&join) {
                        continue;
                    }
                    tree.add(&join);
                }
            }
            if tree.len() == 0 {
                break;
            }
            let k = self.candidates.len() + 1;
            let mut stack = vec![0; k];
            for i in 0..data.data().transactions.len() {
                self.add_to_tree(&mut tree, &data.data().transactions[i], 0, k, &mut stack);
            }
            let mut set = HashSet::new();
            for (arr, n) in tree.iter() {
                if n >= data.min_support() {
                    set.insert(arr.to_vec());
                }
            }
            self.candidates.push(set);
        }
    }
    fn add_to_tree<const N: usize>(
        &self,
        tree: &mut AprioriHashTree<N>,
        data: &[usize],
        i: usize,
        k: usize,
        stack: &mut [usize],
    ) {
        if i == k {
            let mut v = Vec::new();
            for i in stack {
                v.push(data[*i]);
            }
            println!("{v:?}");
            tree.increment(&v);
            return;
        }
        let start = if i == 0 { 0 } else { stack[i - 1] + 1 };
        for j in start..data.len() {
            stack[i] = j;
            self.add_to_tree(tree, data, i + 1, k, stack);
        }
    }
    fn can_be_pruned(&self, v: &[usize]) -> bool {
        let mut arr: Vec<_> = v.iter().cloned().skip(1).collect();
        for i in 0..(v.len() - 2) {
            if !self.candidates[arr.len() - 1].contains(&arr) {
                return true;
            }
            arr[i] = v[i + 1];
        }
        false
    }
    fn run_one(&mut self, data: &Apriori) {
        let mut first = vec![0u64; data.data().num_items];
        for d in data.data().iter() {
            for &item in d {
                first[item] += 1;
            }
        }
        let mut v = HashSet::new();
        for (i, n) in first.into_iter().enumerate() {
            if n >= data.min_support() {
                v.insert(vec![i]);
            }
        }
        self.candidates.push(v);
    }
    fn run_two(&mut self, data: &Apriori) {
        let mut second = Array2D::new(data.data().num_items);
        for d in data.data().iter() {
            for i in 0..d.len() {
                for j in 0..i {
                    second.increment(d[i], d[j]);
                }
            }
        }
        let mut v = HashSet::new();
        for (r, c, count) in second.iter() {
            if count >= data.min_support() {
                v.insert(vec![c, r]);
            }
        }
        self.candidates.push(v);
    }
    
    pub fn candidates(&self) -> &[HashSet<Vec<usize>, RandomState>] {
        &self.candidates
    }
}

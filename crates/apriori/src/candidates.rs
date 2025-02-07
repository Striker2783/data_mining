use std::{collections::HashSet, hash::RandomState};

use datasets::transaction_set::TransactionSet;

use crate::{
    array2d::Array2D, candidates_func::join_tree, hash_tree::AprioriHashTree,
};

#[derive(Debug)]
pub struct Candidates<'a> {
    candidates: Vec<HashSet<Vec<usize>>>,
    data: &'a TransactionSet,
    min_sup: u64,
}

impl<'a> Candidates<'a> {
    pub fn new(data: &'a TransactionSet, min_sup: u64) -> Self {
        Candidates {
            data,
            candidates: Default::default(),
            min_sup,
        }
    }
    pub fn run(&mut self) {
        self.run_one();
        self.run_two();
        while !self.candidates.last().unwrap().is_empty() {
            let c_prev: Vec<_> = self.candidates.last().unwrap().iter().collect();
            let mut tree = join_tree(&c_prev, |v| self.can_be_pruned(v));
            if tree.len() == 0 {
                break;
            }
            let k = self.candidates.len() + 1;
            let mut stack = vec![0; k];
            for i in 0..self.data.transactions.len() {
                Self::add_to_tree(&mut tree, &self.data.transactions[i], 0, k, &mut stack);
            }
            let mut set = HashSet::new();
            for (arr, n) in tree.iter() {
                if n >= self.min_sup {
                    set.insert(arr.to_vec());
                }
            }
            self.candidates.push(set);
        }
    }
    fn add_to_tree<const N: usize>(
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
            tree.increment(&v);
            return;
        }
        let start = if i == 0 { 0 } else { stack[i - 1] + 1 };
        for j in start..data.len() {
            stack[i] = j;
            Self::add_to_tree(tree, data, i + 1, k, stack);
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
    fn run_one(&mut self) {
        let mut first = vec![0u64; self.data.num_items];
        for d in self.data.iter() {
            for &item in d {
                first[item] += 1;
            }
        }
        let mut v = HashSet::new();
        for (i, n) in first.into_iter().enumerate() {
            if n >= self.min_sup {
                v.insert(vec![i]);
            }
        }
        self.candidates.push(v);
    }
    fn run_two(&mut self) {
        let mut second = Array2D::new(self.data.num_items);
        for d in self.data.iter() {
            for i in 0..d.len() {
                for j in 0..i {
                    second.increment(d[i], d[j]);
                }
            }
        }
        let mut v = HashSet::new();
        for (r, c, count) in second.iter() {
            if count >= self.min_sup {
                v.insert(vec![c, r]);
            }
        }
        self.candidates.push(v);
    }

    pub fn candidates(&self) -> &[HashSet<Vec<usize>, RandomState>] {
        &self.candidates
    }
}

#[cfg(test)]
mod tests {
    use datasets::transaction_set::TransactionSet;

    use crate::candidates::Candidates;

    #[test]
    fn test_candidates() {
        let example = TransactionSet::new(
            vec![
                vec![0, 1, 4],
                vec![1, 3],
                vec![1, 2],
                vec![0, 1, 3],
                vec![0, 2],
                vec![1, 2],
                vec![0, 2],
                vec![0, 1, 2, 4],
                vec![0, 1, 2],
            ],
            5,
        );
        let mut candidates = Candidates::new(&example, 2);
        candidates.run();
        assert!(candidates.candidates()[1].contains(&vec![0, 1]));
        assert!(candidates.candidates()[1].contains(&vec![0, 2]));
        assert!(candidates.candidates()[1].contains(&vec![0, 4]));
        assert!(candidates.candidates()[1].contains(&vec![1, 2]));
        assert!(candidates.candidates()[1].contains(&vec![1, 3]));
        assert!(candidates.candidates()[1].contains(&vec![1, 4]));
        assert_eq!(candidates.candidates()[1].len(), 6);
        assert_eq!(candidates.candidates().len(), 3);
        assert_eq!(candidates.candidates()[2].len(), 2);
    }
}

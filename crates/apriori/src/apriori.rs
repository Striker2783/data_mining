use std::ops::{Deref, DerefMut};

use datasets::{transaction_set::TransactionSet, utils::nested_loops};

use crate::{
    array2d::Array2D,
    candidates::{CandidateType, Candidates},
    candidates_func::join,
    hash_tree::AprioriHashTree,
};
#[derive(Debug)]
pub struct Apriori {
    min_support: u64,
}

impl Apriori {
    pub fn new(min_support: u64) -> Self {
        Self { min_support }
    }
    pub fn run(self, data: &TransactionSet) -> Vec<Candidates> {
        let mut v = Vec::new();
        v.push(apriori_run_one(data, self.min_support));
        for i in 2.. {
            let prev = v.last().unwrap();
            let next = AprioriCandidates::new(prev.deref()).run(data, i, self.min_support);
            if next.is_empty() {
                break;
            }
            v.push(next);
        }
        v
    }
}

pub struct AprioriCandidates<T>(T);

impl<T: Deref<Target = CandidateType>> DerefMut for AprioriCandidates<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Deref<Target = CandidateType>> Deref for AprioriCandidates<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Deref<Target = CandidateType>> AprioriCandidates<T> {
    pub fn new(v: T) -> Self {
        Self(v)
    }
    pub fn can_be_pruned(&self, v: &[usize]) -> bool {
        let mut arr: Vec<_> = v.iter().cloned().skip(1).collect();
        if !self.contains(&arr) {
            return true;
        }
        for i in 0..(v.len() - 3) {
            arr[i] = v[i];
            if !self.contains(&arr) {
                return true;
            }
        }
        false
    }
    pub fn run_count(&self, data: &TransactionSet, i: usize) -> AprioriHashTree {
        assert!(i > 2);
        let mut tree = AprioriHashTree::new();
        join(&self.iter().collect::<Vec<_>>(), |v| {
            if self.can_be_pruned(&v) {
                return;
            }
            tree.add(&v);
        });
        for idx in 0..data.transactions.len() {
            let t = &data.transactions[idx];
            if t.len() < i {
                continue;
            }
            let mut combinations = ((t.len() - i + 1).max(i + 1)..=t.len())
                .fold(1usize, |acc, x| acc.saturating_mul(x));
            if combinations != usize::MAX {
                combinations /= (2..(t.len() - i + 1).min(i + 1)).product::<usize>();
            }
            if tree.len() > combinations {
                nested_loops(
                    |v| {
                        tree.increment(v);
                    },
                    &data.transactions[idx],
                    i,
                );
            } else {
                tree.for_each_mut(|v, n| {
                    if v.iter().all(|a| t.contains(a)) {
                        *n += 1;
                    }
                });
            }
        }
        tree
    }
    pub fn run(&self, data: &TransactionSet, i: usize, min_sup: u64) -> Candidates {
        if i == 1 {
            return apriori_run_one(data, min_sup);
        } else if i == 2 {
            return apriori_run_two(data, min_sup);
        }
        let tree = self.run_count(data, i);
        let mut set = Candidates::default();
        for (arr, n) in tree.iter() {
            if n >= min_sup {
                set.insert(arr.to_vec());
            }
        }
        set
    }
}
pub fn apriori_run_one(d: &TransactionSet, min_sup: u64) -> Candidates {
    let mut first = vec![0u64; d.num_items];
    for d in d.iter() {
        for &item in d {
            first[item] += 1;
        }
    }
    let mut v = Candidates::default();
    for (i, n) in first.into_iter().enumerate() {
        if n >= min_sup {
            v.insert(vec![i]);
        }
    }
    v
}
pub fn apriori_run_one_count(d: &TransactionSet) -> Vec<u64> {
    let mut first = vec![0u64; d.num_items];
    for d in d.iter() {
        for &item in d {
            first[item] += 1;
        }
    }
    first
}
pub fn apriori_run_two_count(d: &TransactionSet) -> Array2D<u64> {
    let mut second = Array2D::new(d.num_items);
    for d in d.iter() {
        for i in 0..d.len() {
            for j in 0..i {
                second.increment(d[i], d[j]);
            }
        }
    }
    second
}
pub fn apriori_run_two(d: &TransactionSet, min_sup: u64) -> Candidates {
    let mut second = Array2D::new(d.num_items);
    for d in d.iter() {
        for i in 0..d.len() {
            for j in 0..i {
                second.increment(d[i], d[j]);
            }
        }
    }
    let mut v = Candidates::default();
    for (r, c, count) in second.iter() {
        if count >= min_sup {
            v.insert(vec![c, r]);
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use datasets::transaction_set::TransactionSet;

    use crate::apriori::Apriori;

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
        let apriori = Apriori::new(2);
        let result = apriori.run(&example);
        assert!(result[1].contains(&vec![0, 1]));
        assert!(result[1].contains(&vec![0, 2]));
        assert!(result[1].contains(&vec![0, 4]));
        assert!(result[1].contains(&vec![1, 2]));
        assert!(result[1].contains(&vec![1, 3]));
        assert!(result[1].contains(&vec![1, 4]));
        assert_eq!(result[1].len(), 6);
        assert_eq!(result.len(), 3);
        assert_eq!(result[2].len(), 2);
    }
}

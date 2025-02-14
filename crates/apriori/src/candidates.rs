use std::collections::HashSet;

use datasets::{transaction_set::TransactionSet, utils::nested_loops};

use crate::{
    CandidateType, apriori_hybrid::BasicCandidates, array2d::Array2D, candidates_func::join,
    hash_tree::AprioriHashTree,
};
#[derive(Debug, Default)]
pub struct Candidates {
    data: CandidateType,
}

impl Candidates {
    pub fn new(data: CandidateType) -> Self {
        Self { data }
    }

    pub fn next_i(s: &CandidateType, data: &TransactionSet, i: usize, min_sup: u64) -> Candidates {
        if i == 1 {
            Self::run_one(data, min_sup)
        } else if i == 2 {
            Self::run_two(data, min_sup)
        } else {
            let mut tree = AprioriHashTree::<50>::new();
            join(&s.iter().collect::<Vec<_>>(), |v| {
                if Self::can_be_pruned(s, &v) {
                    return;
                }
                tree.add(&v);
            });
            for idx in 0..data.transactions.len() {
                nested_loops(|v| tree.increment(&v), &data.transactions[idx], i);
            }
            let mut set = HashSet::new();
            for (arr, n) in tree.iter() {
                if n >= min_sup {
                    set.insert(arr.to_vec());
                }
            }
            Self::new(set)
        }
    }

    pub fn next(&self, data: &TransactionSet, i: usize, min_sup: u64) -> Self {
        Self::next_i(&self.data, data, i, min_sup)
    }
    fn can_be_pruned(data: &CandidateType, v: &[usize]) -> bool {
        let mut arr: Vec<_> = v.iter().cloned().skip(1).collect();
        for i in 0..(v.len() - 2) {
            if !data.contains(&arr) {
                return true;
            }
            arr[i] = v[i + 1];
        }
        false
    }
    pub fn run_one(data: &TransactionSet, min_sup: u64) -> Self {
        let mut first = vec![0u64; data.num_items];
        for d in data.iter() {
            for &item in d {
                first[item] += 1;
            }
        }
        let mut v = HashSet::new();
        for (i, n) in first.into_iter().enumerate() {
            if n >= min_sup {
                v.insert(vec![i]);
            }
        }
        Self::new(v)
    }
    fn run_two(data: &TransactionSet, min_sup: u64) -> Self {
        let mut second = Array2D::new(data.num_items);
        for d in data.iter() {
            for i in 0..d.len() {
                for j in 0..i {
                    second.increment(d[i], d[j]);
                }
            }
        }
        let mut v = HashSet::new();
        for (r, c, count) in second.iter() {
            if count >= min_sup {
                v.insert(vec![c, r]);
            }
        }
        Self::new(v)
    }

    pub fn data(&self) -> &CandidateType {
        &self.data
    }

    pub fn data_owned(self) -> CandidateType {
        self.data
    }
}
impl From<BasicCandidates> for Candidates {
    fn from(value: BasicCandidates) -> Self {
        Self::new(value.0)
    }
}

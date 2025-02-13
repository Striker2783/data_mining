use std::collections::HashSet;

use datasets::{transaction_set::TransactionSet, utils::nested_loops};

use crate::{array2d::Array2D, candidates_func::join, hash_tree::AprioriHashTree};

pub struct CandidatesList {
    candidates: Vec<Candidates>,
    min_sup: u64,
}

impl CandidatesList {
    pub fn new(min_sup: u64) -> Self {
        Self {
            candidates: Vec::new(),
            min_sup,
        }
    }

    pub fn run_apriori(&mut self, data: &TransactionSet) {
        let empty = Candidates::default();
        for i in 1.. {
            let prev = self.candidates.last().unwrap_or(&empty);
            let candidates = prev.next(prev, &data, i, self.min_sup);
            if candidates.data().is_empty() {
                break;
            }
            self.candidates.push(candidates);
        }
    }

    pub fn candidates(&self) -> &[Candidates] {
        &self.candidates
    }
}

#[derive(Debug, Default)]
pub struct Candidates {
    data: HashSet<Vec<usize>>,
}

impl Candidates {
    pub fn new(data: HashSet<Vec<usize>>) -> Self {
        Self { data }
    }

    pub fn next(
        &self,
        prev: &Self,
        data: &TransactionSet,
        i: usize,
        min_sup: u64,
    ) -> Self {
        if i == 1 {
            Self::run_one(data, min_sup)
        } else if i == 2 {
            Self::run_two(data, min_sup)
        } else {
            let mut tree = AprioriHashTree::<50>::new();
            join(&prev.data.iter().collect::<Vec<_>>(), |v| {
                if self.can_be_pruned(prev, &v) {
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
    fn can_be_pruned(&self, prev: &Self, v: &[usize]) -> bool {
        let mut arr: Vec<_> = v.iter().cloned().skip(1).collect();
        for i in 0..(v.len() - 2) {
            if !prev.data.contains(&arr) {
                return true;
            }
            arr[i] = v[i + 1];
        }
        false
    }
    fn run_one(data: &TransactionSet, min_sup: u64) -> Self {
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

    pub fn data(&self) -> &HashSet<Vec<usize>> {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use datasets::transaction_set::TransactionSet;

    use crate::candidates::CandidatesList;

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
        let mut candidates = CandidatesList::new(2);
        candidates.run_apriori(&example);
        assert!(candidates.candidates()[1].data().contains(&vec![0, 1]));
        assert!(candidates.candidates()[1].data().contains(&vec![0, 2]));
        assert!(candidates.candidates()[1].data().contains(&vec![0, 4]));
        assert!(candidates.candidates()[1].data().contains(&vec![1, 2]));
        assert!(candidates.candidates()[1].data().contains(&vec![1, 3]));
        assert!(candidates.candidates()[1].data().contains(&vec![1, 4]));
        assert_eq!(candidates.candidates()[1].data().len(), 6);
        assert_eq!(candidates.candidates().len(), 3);
        assert_eq!(candidates.candidates()[2].data().len(), 2);
    }
}

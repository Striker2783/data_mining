use std::collections::HashSet;

use datasets::{transaction_set::TransactionSet, utils::nested_loops};

use crate::{array2d::Array2D, candidates_func::join, hash_tree::AprioriHashTree};

pub struct CandidatesList {
    candidates: Vec<HashSet<Vec<usize>>>,
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
        let empty = HashSet::new();
        for i in 1.. {
            let prev = self.candidates.last().unwrap_or(&empty);
            let candidates = Candidates::new(&data.transactions, prev, self.min_sup);
            let result = candidates.run(i, data.num_items);
            if result.is_empty() {
                break;
            }
            self.candidates.push(result);
        }
    }

    pub fn candidates(&self) -> &[HashSet<Vec<usize>>] {
        &self.candidates
    }
}

#[derive(Debug)]
pub struct Candidates<'a> {
    prev_candidates: &'a HashSet<Vec<usize>>,
    data: &'a [Vec<usize>],
    min_sup: u64,
}

impl<'a> Candidates<'a> {
    pub fn new(data: &'a [Vec<usize>], prev: &'a HashSet<Vec<usize>>, min_sup: u64) -> Self {
        Candidates {
            data,
            prev_candidates: prev,
            min_sup,
        }
    }
    pub fn run(mut self, i: usize, n: usize) -> HashSet<Vec<usize>> {
        if i == 1 {
            self.run_one(n)
        } else if i == 2 {
            self.run_two(n)
        } else {
            let mut tree = AprioriHashTree::<50>::new();
            join(&self.prev_candidates.iter().collect::<Vec<_>>(), |v| {
                if self.can_be_pruned(&v) {
                    return;
                }
                tree.add(&v);
            });
            for idx in 0..self.data.len() {
                nested_loops(|v| tree.increment(&v), &self.data[idx], i);
            }
            let mut set = HashSet::new();
            for (arr, n) in tree.iter() {
                if n >= self.min_sup {
                    set.insert(arr.to_vec());
                }
            }
            set
        }
    }
    fn can_be_pruned(&self, v: &[usize]) -> bool {
        let mut arr: Vec<_> = v.iter().cloned().skip(1).collect();
        for i in 0..(v.len() - 2) {
            if !self.prev_candidates.contains(&arr) {
                return true;
            }
            arr[i] = v[i + 1];
        }
        false
    }
    fn run_one(&mut self, n: usize) -> HashSet<Vec<usize>> {
        let mut first = vec![0u64; n];
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
        v
    }
    fn run_two(&mut self, n: usize) -> HashSet<Vec<usize>> {
        let mut second = Array2D::new(n);
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
        v
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

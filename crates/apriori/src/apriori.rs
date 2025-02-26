use std::collections::HashSet;

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
        v.push(run_one(data, self.min_support));
        for i in 2.. {
            let prev = v.last().unwrap();
            let next = run(prev, data, i, self.min_support);
            if next.is_empty() {
                break;
            }
            v.push(next);
        }
        v
    }
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
pub fn run(s: &CandidateType, data: &TransactionSet, i: usize, min_sup: u64) -> Candidates {
    assert!(i >= 2);
    if i == 2 {
        return run_two(data, min_sup);
    }
    let mut tree = AprioriHashTree::<50>::new();
    join(&s.iter().collect::<Vec<_>>(), |v| {
        if can_be_pruned(s, &v) {
            return;
        }
        tree.add(&v);
    });
    for idx in 0..data.transactions.len() {
        nested_loops(|v| tree.increment(v), &data.transactions[idx], i);
    }
    let mut set = HashSet::new();
    for (arr, n) in tree.iter() {
        if n >= min_sup {
            set.insert(arr.to_vec());
        }
    }
    Candidates::new(set)
}
pub fn run_two(d: &TransactionSet, min_sup: u64) -> Candidates {
    let mut second = Array2D::new(d.num_items);
    for d in d.iter() {
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
    Candidates::new(v)
}
pub fn run_one(d: &TransactionSet, min_sup: u64) -> Candidates {
    let mut first = vec![0u64; d.num_items];
    for d in d.iter() {
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
    Candidates::new(v)
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

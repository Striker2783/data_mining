use std::ops::{Deref, DerefMut};

use datasets::transaction_set::TransactionSet;

use crate::{
    apriori::apriori_run_one, candidates::{CandidateType, Candidates}, candidates_func::join, hash_tree::AprioriHashTree2, transaction_id::TransactionIDs
};

pub struct AprioriTID {
    min_support: u64,
}

impl AprioriTID {
    pub fn new(min_support: u64) -> Self {
        Self { min_support }
    }
    pub fn run(&self, data: &TransactionSet) -> Vec<Candidates> {
        let mut v = vec![apriori_run_one(data, self.min_support)];
        let mut prev_transactions = TransactionIDs::from(data);
        loop {
            let prev = v.last().unwrap();
            let next =
                AprioriTiDCandidates::new(prev.deref()).next(&prev_transactions, self.min_support);
            if next.is_empty() {
                break;
            }
            prev_transactions = prev_transactions.from_prev(&next);
            v.push(next);
        }
        v
    }
}

pub struct AprioriTiDCandidates<T: Deref<Target = CandidateType>>(T);

impl<T: Deref<Target = CandidateType>> AprioriTiDCandidates<T> {
    pub fn new(v: T) -> Self {
        Self(v)
    }
    pub fn next(&self, data: &TransactionIDs, min_sup: u64) -> Candidates {
        let mut tree = AprioriHashTree2::new();
        join(&self.0.iter().collect::<Vec<_>>(), |join| {
            tree.add(&join);
        });
        data.count(tree.deref_mut());
        let mut new_candidates = Candidates::default();
        tree.iter().for_each(|(v, n)| {
            if n < min_sup {
                return;
            }
            new_candidates.insert(v.to_vec());
        });
        new_candidates
    }
}

#[cfg(test)]
mod tests {
    use datasets::transaction_set::TransactionSet;

    use super::AprioriTID;

    #[test]
    fn test_apriori_tid() {
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
        let apriori = AprioriTID::new(2);
        let result = apriori.run(&example);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 5);
        assert_eq!(result[1].len(), 6);
        assert_eq!(result[2].len(), 2);
    }
}

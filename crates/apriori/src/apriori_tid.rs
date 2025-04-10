use std::ops::{Deref, DerefMut};

use datasets::transaction_set::TransactionSet;

use crate::{
    apriori::{AprioriCandidates, apriori_run_one},
    candidates::{CandidateType, Candidates},
    hash_tree::AprioriHashTree,
    transaction_id::TransactionIDs,
};
/// The AprioriTID algorithm
pub struct AprioriTID {
    min_support: u64,
}

impl AprioriTID {
    /// Constructor
    pub fn new(min_support: u64) -> Self {
        Self { min_support }
    }
    /// Runs the algorithm
    pub fn run(&self, data: &TransactionSet) -> Vec<Candidates> {
        // Gets all the frequent items
        let mut v = vec![apriori_run_one(data, self.min_support)];
        // Generates the TIDs
        let mut prev_transactions = TransactionIDs::from(data);
        loop {
            let prev = v.last().unwrap();
            // Generates the frequent itemsets
            let next =
                AprioriTiDCandidates::new(prev.deref()).next(&prev_transactions, self.min_support);
            if next.is_empty() {
                break;
            }
            // Generates the next TIDs
            prev_transactions = prev_transactions.from_prev(&next);
            v.push(next);
        }
        v
    }
    /// Runs the algorithm a different (but proper) way, but slower
    pub fn run_obsolete(&self, data: &TransactionSet) -> Vec<Candidates> {
        // Gets all the frequent items
        let mut v = vec![apriori_run_one(data, self.min_support)];
        // Generates the TIDs
        let mut prev_transactions = TransactionIDs::from(data);
        loop {
            let prev = v.last().unwrap();
            // Finds the frequent itemsets and next TIDs
            let (next, next_t) = AprioriTiDCandidates::new(prev.deref())
                .next_with_next(&prev_transactions, self.min_support);
            if next.is_empty() {
                break;
            }
            prev_transactions = next_t;
            v.push(next);
        }
        v
    }
}
/// Contains the algorithm for AprioriTID
pub struct AprioriTiDCandidates<'a>(&'a CandidateType);

impl<'a> AprioriTiDCandidates<'a> {
    pub fn new(v: &'a CandidateType) -> Self {
        Self(v)
    }
    /// Generates the frequent itemsets and next TIDs
    pub fn next_with_next(
        &self,
        data: &TransactionIDs,
        min_sup: u64,
    ) -> (Candidates, TransactionIDs) {
        let (tree, next) = self.count_with_next(data);
        // Returns the new frequent itemsets
        let mut new_candidates = Candidates::default();
        tree.iter().for_each(|(v, n)| {
            if n < min_sup {
                return;
            }
            new_candidates.insert(v.to_vec());
        });
        (new_candidates, next)
    }
    /// Generates the counts for candidate itemsets and next TIDs
    pub fn count_with_next(&self, data: &TransactionIDs) -> (AprioriHashTree, TransactionIDs) {
        let mut tree = AprioriCandidates::new(self.0).create_tree();
        // Counts the TIDs and generates the next ones
        let next = data.count_with_next(tree.deref_mut());
        (tree, next)
    }
    /// Generates the frequent itemsets
    pub fn next(&self, data: &TransactionIDs, min_sup: u64) -> Candidates {
        let tree = self.next_count(data);
        // Returns the frequent itemsets
        let mut new_candidates = Candidates::default();
        tree.iter().for_each(|(v, n)| {
            if n < min_sup {
                return;
            }
            new_candidates.insert(v.to_vec());
        });
        new_candidates
    }
    /// Generates the counts into a tree
    pub fn next_count(&self, data: &TransactionIDs) -> AprioriHashTree {
        let mut tree = AprioriCandidates::new(self.0).create_tree();
        // Counting
        data.count(tree.deref_mut());
        tree
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
        println!("{result:?}");
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 5);
        assert_eq!(result[1].len(), 6);
        assert_eq!(result[2].len(), 2);
    }
}

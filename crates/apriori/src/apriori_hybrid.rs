use std::ops::Deref;

use datasets::transaction_set::TransactionSet;

use crate::{
    apriori::{AprioriCandidates, apriori_run_one},
    apriori_tid::AprioriTiDCandidates,
    candidates::Candidates,
    transaction_id::TransactionIDs,
};
/// The AprioriHybrid Algorithm
pub struct AprioriHybrid {
    min_support: u64,
    /// When to switch to using AprioriTID
    switch: usize,
}

impl AprioriHybrid {
    /// Constructor
    pub fn new(min_support: u64, switch: usize) -> Self {
        AprioriHybrid {
            min_support,
            switch,
        }
    }
    /// Runs the algorithm
    pub fn run(&self, data: &TransactionSet) -> Vec<Candidates> {
        // The frequent itemsets generated
        let mut apriori = vec![apriori_run_one(data, self.min_support)];
        let mut prev_trans = TransactionIDs::default();
        for i in 2.. {
            // When we switch, we generate TIDs from the transaction set
            if i == self.switch {
                let prev = apriori.last().unwrap();
                let (tree, a) =
                    TransactionIDs::from_transaction(&data.transactions, i - 1, prev.deref());
                let mut c = Candidates::default();
                tree.iter().for_each(|(v, count)| {
                    if count < self.min_support {
                        return;
                    }
                    c.insert(v.to_vec());
                });
                prev_trans = a;
                apriori.push(c);
                continue;
            }
            // Apriori
            if i < self.switch {
                let prev = apriori.last().unwrap();
                let next = AprioriCandidates::new(prev.deref()).run(data, i, self.min_support);
                if next.is_empty() {
                    break;
                }
                apriori.push(next);
            } else {
                // AprioriTID
                let prev = apriori.last().unwrap();
                let (next, b) =
                    AprioriTiDCandidates::new(prev.deref()).next(&prev_trans, self.min_support);
                if next.is_empty() {
                    break;
                }
                prev_trans = b;
                apriori.push(next);
            }
        }
        apriori
    }
    /// Runs the algorithm
    pub fn run_fn(&self, data: &TransactionSet, mut f: impl FnMut(&[usize])) {
        // The frequent itemsets generated
        let mut prev = apriori_run_one(data, self.min_support);
        prev.iter().for_each(|v| f(v));
        let mut prev_trans = TransactionIDs::default();
        for i in 2.. {
            // When we switch, we generate TIDs from the transaction set
            if i == self.switch {
                let (tree, a) =
                    TransactionIDs::from_transaction(&data.transactions, i - 1, prev.deref());
                let mut c = Candidates::default();
                tree.iter().for_each(|(v, count)| {
                    if count < self.min_support {
                        return;
                    }
                    c.insert(v.to_vec());
                });
                prev_trans = a;
                prev = c;
                prev.iter().for_each(|v| f(v));
                continue;
            }
            // Apriori
            if i < self.switch {
                let next = AprioriCandidates::new(prev.deref()).run(data, i, self.min_support);
                if next.is_empty() {
                    break;
                }
                prev = next;
                prev.iter().for_each(|v| f(v));
            } else {
                // AprioriTID
                let (next, b) =
                    AprioriTiDCandidates::new(prev.deref()).next(&prev_trans, self.min_support);
                if next.is_empty() {
                    break;
                }
                prev_trans = b;
                prev = next;
                prev.iter().for_each(|v| f(v));
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use datasets::transaction_set::TransactionSet;

    use crate::apriori_hybrid::AprioriHybrid;

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
        let apriori = AprioriHybrid::new(2, 3);
        let result = apriori.run(&example);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 5);
        assert_eq!(result[1].len(), 6);
        assert_eq!(result[2].len(), 2);
    }
}

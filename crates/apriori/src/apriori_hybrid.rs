use std::ops::Deref;

use datasets::transaction_set::TransactionSet;

use crate::{
    apriori::{AprioriCandidates, apriori_run_one},
    candidates::Candidates,
    candidates_tid::AprioriTiDCandidates,
    transaction_id::TransactionIDs,
};

pub struct AprioriHybrid {
    min_support: u64,
    switch: usize,
}

impl AprioriHybrid {
    pub fn new(min_support: u64, switch: usize) -> Self {
        AprioriHybrid {
            min_support,
            switch,
        }
    }
    pub fn run(&self, data: &TransactionSet) -> Vec<Candidates> {
        let mut apriori = vec![apriori_run_one(data, self.min_support)];
        let mut apriori_tid = Vec::new();
        let mut prev_trans = TransactionIDs::default();
        for i in 2.. {
            if i == self.switch {
                let prev = apriori.pop().unwrap();
                prev_trans = TransactionIDs::from_transaction(&data.transactions, i - 1, &prev);
                apriori_tid.push(prev);
            }
            if i < self.switch {
                let prev = apriori.last().unwrap();
                let next = AprioriCandidates::new(prev.deref()).run(data, i, self.min_support);
                if next.is_empty() {
                    break;
                }
                apriori.push(next);
            } else {
                let prev = apriori_tid.last().unwrap();
                let next =
                    AprioriTiDCandidates::new(prev.deref()).next(&prev_trans, self.min_support);
                if next.is_empty() {
                    break;
                }
                prev_trans = prev_trans.from_prev(&next);
                apriori_tid.push(next);
            }
        }
        let mut v = Vec::new();
        for a in apriori {
            v.push(a);
        }
        for a in apriori_tid {
            v.push(a);
        }
        v
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
        let apriori = AprioriHybrid::new(2, 2);
        let result = apriori.run(&example);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 5);
        assert_eq!(result[1].len(), 6);
        assert_eq!(result[2].len(), 2);
    }
}

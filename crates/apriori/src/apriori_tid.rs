use datasets::transaction_set::TransactionSet;

use crate::{apriori::run_one, candidate::Candidates, candidates_tid::next, transaction_id::TransactionIDs};

pub struct AprioriTID {
    min_support: u64,
}

impl AprioriTID {
    pub fn new(min_support: u64) -> Self {
        Self { min_support }
    }
    pub fn run(&self, data: &TransactionSet) -> Vec<Candidates> {
        let mut v = vec![run_one(data, self.min_support)];
        let mut prev_transactions = TransactionIDs::from(data);
        loop {
            let prev = v.last().unwrap();
            let next = next(prev, &prev_transactions, self.min_support);
            if next.is_empty() {
                break;
            }
            prev_transactions = prev_transactions.from_prev(&next);
            v.push(next);
        }
        v
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

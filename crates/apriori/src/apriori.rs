
use datasets::transaction_set::TransactionSet;

use crate::candidates::Candidates;
#[derive(Debug)]
pub struct Apriori<'a> {
    data: &'a TransactionSet,
    min_support: u64,
    min_confidence: u64,
}

impl<'a> Apriori<'a> {
    pub fn new(data: &'a TransactionSet, min_support: u64, min_confidence: u64) -> Self {
        Self {
            data,
            min_support,
            min_confidence,
        }
    }
    pub fn run(self) {}
    fn get_candidates(&self) -> Candidates {
        let mut candidates = Candidates::new();
        candidates.run(&self);
        candidates
    }
    
    pub fn min_support(&self) -> u64 {
        self.min_support
    }
    
    pub fn min_confidence(&self) -> u64 {
        self.min_confidence
    }
    
    pub fn data(&self) -> &TransactionSet {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_get_candidates() {
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
        let apriori = Apriori::new(&example, 2, 1);
        let candidates = apriori.get_candidates();
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

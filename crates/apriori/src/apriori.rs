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
        let mut candidates = Candidates::new(self.data(), self.min_support);
        candidates.run();
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

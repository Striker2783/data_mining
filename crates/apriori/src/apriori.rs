use datasets::transaction_set::TransactionSet;

use crate::candidates::CandidatesList;
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
    pub fn run(self) -> CandidatesList {
        let mut candidates = CandidatesList::new(self.min_support);
        candidates.run_apriori(self.data());
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

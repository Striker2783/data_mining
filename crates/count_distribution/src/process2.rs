use std::sync::Arc;

use apriori::candidates::CandidateType;
use datasets::transaction_set::TransactionSet;

pub struct CDProcess {
    data: Arc<TransactionSet>,
    candidates: Arc<CandidateType>,
}

impl CDProcess {
    pub fn new(data: Arc<TransactionSet>, candidates: Arc<CandidateType>) -> Self {
        Self { data, candidates }
    }
    pub fn run(self, n: usize) {
        
    }
}

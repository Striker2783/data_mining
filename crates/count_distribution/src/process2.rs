use std::{ops::Deref, sync::Arc};

use apriori::{apriori::{apriori_run_one_count, apriori_run_two_count, AprioriCandidates}, array2d::Array2D, candidates::{CandidateType, Candidates}};
use datasets::transaction_set::TransactionSet;

pub struct CDProcess {
    data: Arc<TransactionSet>,
    candidates: Arc<Candidates>,
}

impl CDProcess {
    pub fn new(data: Arc<TransactionSet>, candidates: Arc<Candidates>) -> Self {
        Self { data, candidates }
    }
    pub fn run(self, n: usize) -> apriori::hash_tree::AprioriHashTree {
        assert!(n > 2);
        AprioriCandidates::new(self.candidates.deref().deref()).run_count(&self.data, n)
    }
    pub fn run_one(self) -> Vec<u64> {
        apriori_run_one_count(&self.data)
    }
    pub fn run_two(self) -> Array2D<u64> {
        apriori_run_two_count(&self.data)
    }
}

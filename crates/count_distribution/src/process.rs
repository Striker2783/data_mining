use std::{ops::Deref, sync::Arc};

use apriori::{apriori::{apriori_run_one_count, apriori_run_two_count, AprioriCandidates}, array2d::Array2D, candidates::Candidates};
use datasets::transaction_set::TransactionSet;
/// A thread for Count Distribution
pub struct CDProcess {
    /// The partition the thread will use
    data: Arc<TransactionSet>,
    /// The candidates from the previous passes
    candidates: Arc<Candidates>,
}

impl CDProcess {
    /// Constructor
    pub fn new(data: Arc<TransactionSet>, candidates: Arc<Candidates>) -> Self {
        Self { data, candidates }
    }
    /// Runs the algorithm for passes 3+
    pub fn run(self, n: usize) -> apriori::hash_tree::AprioriHashTree {
        assert!(n > 2);
        AprioriCandidates::new(self.candidates.deref().deref()).run_count(&self.data, n)
    }
    /// Runs the algorithm for pass 1
    pub fn run_one(self) -> Vec<u64> {
        apriori_run_one_count(&self.data)
    }
    /// Runs the algorithm for pass 2
    pub fn run_two(self) -> Array2D<u64> {
        apriori_run_two_count(&self.data)
    }
}

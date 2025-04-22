use std::ops::Deref;

use apriori::{
    apriori::{AprioriCandidates, apriori_run_one_count, apriori_run_two_count},
    apriori_tid::AprioriTiDCandidates,
    candidates::{CandidateCounter, Candidates},
    transaction_id::TransactionIDs,
};
use datasets::transaction_set::TransactionSet;
/// The thread for Count Distribution Hybrid
pub struct CDProcessHybrid {
    /// The partitioned set
    set: TransactionSet,
    /// The Transaction IDs
    tid: TransactionIDs,
    /// The pass to switch
    switch: usize,
}

impl CDProcessHybrid {
    /// Constructor
    pub fn new(set: TransactionSet, switch: usize) -> Self {
        Self {
            set,
            tid: TransactionIDs::default(),
            switch,
        }
    }
    /// Runs the counting
    pub fn run(&mut self, n: usize, c: &Candidates) -> CandidateCounter {
        if n == 1 {
            // Pass 1 counting and transforming it into a map
            let c = apriori_run_one_count(&self.set);
            let counter = c
                .into_iter()
                .enumerate()
                .map(|(i, v)| (vec![i], v))
                .collect();
            return counter;
        } else if n == 2 {
            // Pass 2 counting and transforming it into a map
            let c = apriori_run_two_count(&self.set);
            let counter = c.iter().map(|(r, c, v)| (vec![c, r], v)).collect();
            return counter;
        }
        if n == self.switch {
            let (c, b) = TransactionIDs::from_transaction(&self.set.transactions, n - 1, c);
            // At the switch, create the TIDs from the transactions
            self.tid = b;
            let map = c.iter().map(|(v, n)| (v.to_vec(), n)).collect();
            return map;
        } else if n > self.switch {
            let (c, b) = AprioriTiDCandidates::new(c.deref()).count(&self.tid);
            self.tid = b;
            // AprioriTID counting after the switch
            c.iter().map(|(v, n)| (v.to_vec(), n)).collect()
        } else {
            // Apriori counting before the switch
            let c = AprioriCandidates::new(c.deref()).run_count(&self.set, n);
            c.iter().map(|(v, n)| (v.to_vec(), n)).collect()
        }
    }
}

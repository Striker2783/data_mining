use std::ops::Deref;

use apriori::{
    apriori::{AprioriCandidates, apriori_run_one_count, apriori_run_two_count},
    apriori_tid::AprioriTiDCandidates,
    candidates::{CandidateCounter, Candidates},
    transaction_id::TransactionIDs,
};
use datasets::transaction_set::TransactionSet;

pub struct CDProcessHybrid {
    set: TransactionSet,
    tid: TransactionIDs,
    switch: usize,
}

impl CDProcessHybrid {
    pub fn new(set: TransactionSet, switch: usize) -> Self {
        Self {
            set,
            tid: TransactionIDs::default(),
            switch,
        }
    }
    pub fn run(&mut self, n: usize, c: &Candidates) -> CandidateCounter {
        if n == 1 {
            let c = apriori_run_one_count(&self.set);
            let counter = c
                .into_iter()
                .enumerate()
                .map(|(i, v)| (vec![i], v))
                .collect();
            return counter;
        } else if n == 2 {
            let c = apriori_run_two_count(&self.set);
            let counter = c.iter().map(|(r, c, v)| (vec![c, r], v)).collect();
            return counter;
        }
        if self.switch == n {
            self.tid = TransactionIDs::from_transaction(&self.set.transactions, n - 1, c);
        } else if self.switch > n {
            self.tid = self.tid.from_prev(c);
        }
        if self.switch >= n {
            let c = AprioriTiDCandidates::new(c.deref()).next_count(&self.tid);
            let map = c.iter().map(|(v, n)| (v.to_vec(), n)).collect();
            return map;
        } else {
            let c = AprioriCandidates::new(c.deref()).run_count(&self.set, n);
            let map = c.iter().map(|(v, n)| (v.to_vec(), n)).collect();
            return map;
        }
    }
}

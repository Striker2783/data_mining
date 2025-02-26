use std::ops::{Deref, DerefMut};


use crate::{
    candidates::{CandidateType, Candidates}, candidates_func::join, hash_tree::AprioriHashTree2, transaction_id::TransactionIDs
};

pub struct AprioriTiDCandidates<T: Deref<Target = CandidateType>>(T);

impl<T: Deref<Target = CandidateType>> AprioriTiDCandidates<T> {
    pub fn new(v: T) -> Self {
        Self(v)
    }
    pub fn next(&self, data: &TransactionIDs, min_sup: u64) -> Candidates {
        let mut tree = AprioriHashTree2::new();
        join(&self.0.iter().collect::<Vec<_>>(), |join| {
            tree.add(&join);
        });
        data.count(tree.deref_mut());
        let mut new_candidates = Candidates::default();
        tree.iter().for_each(|(v, n)| {
            if n < min_sup {
                return;
            }
            new_candidates.insert(v.to_vec());
        });
        new_candidates
    }
}


use std::collections::HashSet;

use datasets::transaction_set::TransactionSet;

use crate::{
    CandidateType, apriori_hybrid::BasicCandidates, candidates::Candidates, candidates_func::join,
    hash_tree::AprioriHashTree, transaction_id::TransactionIDs,
};

#[derive(Debug)]
pub struct CandidateTid {
    candidates: CandidateType,
}
impl CandidateTid {
    pub fn new(candidates: CandidateType) -> Self {
        Self { candidates }
    }
    pub fn next_i(s: &CandidateType, data: &TransactionIDs, min_sup: u64) -> Self {
        let mut tree: AprioriHashTree<50> = AprioriHashTree::new();
        join(&s.iter().collect::<Vec<_>>(), |join| {
            tree.add(&join);
        });
        data.count(&mut tree);
        let mut new_candidates = HashSet::new();
        tree.iter().for_each(|(v, n)| {
            if n < min_sup {
                return;
            }
            new_candidates.insert(v.to_vec());
        });
        Self::new(new_candidates)
    }
    pub fn next(&self, data: &TransactionIDs, min_sup: u64) -> Self {
        Self::next_i(&self.candidates, data, min_sup)
    }
    pub fn one(data: &TransactionSet, min_sup: u64) -> Self {
        Self::from(BasicCandidates::from(Candidates::run_one(data, min_sup)))
    }

    pub fn candidates(&self) -> &CandidateType {
        &self.candidates
    }

    pub fn candidates_owned(self) -> CandidateType {
        self.candidates
    }
}
impl From<BasicCandidates> for CandidateTid {
    fn from(candidates: BasicCandidates) -> Self {
        Self::new(candidates.0)
    }
}

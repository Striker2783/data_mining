use std::collections::HashSet;

pub mod hash_tree;
pub mod apriori;
pub mod array2d;
pub mod candidates;
pub mod candidates_func;
pub mod transaction_id;
pub mod candidates_tid;
pub mod apriori_tid;
pub mod apriori_hybrid;
pub mod candidate;

pub type CandidateType = HashSet<Vec<usize>>;
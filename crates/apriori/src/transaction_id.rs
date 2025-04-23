use std::{collections::{HashMap, HashSet}, ops::DerefMut};

use datasets::{transaction_set::TransactionSet, utils::nested_loops};

use crate::{apriori::{apriori_count, AprioriCandidates}, candidates_func::join, hash_tree::AprioriHashTree};
/// The transaction IDs used for AprioriTID
#[derive(Debug, Default)]
pub struct TransactionIDs {
    v: Vec<TransactionID>,
}

impl TransactionIDs {
    pub fn new(v: Vec<TransactionID>) -> Self {
        Self { v }
    }
    /// Counts using TID into set, and returns the next set of TIDs
    pub fn count<T: TransactionIdCounts>(&self, set: &mut T) -> Self {
        let mut o = Self::default();
        for d in &self.v {
            let a = d.count(set);
            o.v.push(a);
        }
        o
    }
    /// Creates the set of TIDs for the first pass only
    pub fn start(data: &Vec<Vec<usize>>) -> TransactionIDs {
        let mut v = Vec::new();
        for d in data {
            let value = TransactionID::start(d);
            if value.ids().is_empty() {
                continue;
            }
            v.push(value);
        }
        Self::new(v)
    }
    /// Generates the TIDs of size k
    pub fn from_transaction(
        data: &Vec<Vec<usize>>,
        k: usize,
        set: &HashSet<Vec<usize>>,
    ) -> (AprioriHashTree, Self) {
        let mut tree = AprioriCandidates::new(set).create_tree();
        let mut v = Vec::new();
        for d in data {
            let value = TransactionID::from_transaction(d, k, &mut tree);
            if value.ids().is_empty() {
                continue;
            }
            v.push(value);
        }
        (tree, Self::new(v))
    }
}
impl From<&TransactionSet> for TransactionIDs {
    fn from(transaction_set: &TransactionSet) -> Self {
        Self::start(&transaction_set.transactions)
    }
}
/// A Transaction ID for AprioriTID
#[derive(Debug, Default)]
pub struct TransactionID {
    v: HashSet<Vec<usize>>,
}

impl TransactionID {
    pub fn new(v: HashSet<Vec<usize>>) -> Self {
        Self { v }
    }
    /// Counts the itemsets into set, and returns the next TID
    pub fn count<T: TransactionIdCounts>(&self, set: &mut T) -> Self {
        // Count through joining together IDs
        let mut t = TransactionID::default();
        join(self.v.iter(), |v| {
            if set.increment(&v) {
                t.v.insert(v);
            }
        });
        t
    }
    /// Generates the TID for pass 1
    pub fn start(data: &[usize]) -> Self {
        Self::new(data.iter().cloned().map(|n| vec![n]).collect())
    }
    /// Generates the next TID from the dataset for size k
    pub fn from_transaction(data: &[usize], k: usize, set: &mut AprioriHashTree) -> Self {
        // Generates the TID based on nested looping through the transaction set.
        let mut output = HashSet::new();
        apriori_count(data, k + 1, set.deref_mut(), |v| {
            output.insert(v.to_vec());
        });
        Self { v: output }
    }
    pub fn ids(&self) -> &HashSet<Vec<usize>> {
        &self.v
    }
    pub fn ids_mut(&mut self) -> &mut HashSet<Vec<usize>> {
        &mut self.v
    }
}
/// A trait used for TID counting
pub trait TransactionIdCounts {
    /// Increments the count
    fn increment(&mut self, v: &[usize]) -> bool;
    /// Gets the length of the count
    fn len(&self) -> usize;
    /// A for each loop through the counter
    fn for_each(&self, f: impl FnMut(&[usize]));
    /// Checks if the counter is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
impl TransactionIdCounts for HashMap<Vec<usize>, u64> {
    fn increment(&mut self, v: &[usize]) -> bool {
        if let Some(a) = self.get_mut(v) {
            *a += 1;
            true
        } else {
            false
        }
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn for_each(&self, mut f: impl FnMut(&[usize])) {
        self.iter().for_each(|v| f(v.0));
    }
}

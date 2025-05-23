use std::ops::{Deref, DerefMut};

use datasets::{transaction_set::TransactionSet, utils::nested_loops};

use crate::{
    array2d::Array2D,
    candidates::{CandidateType, Candidates},
    candidates_func::join,
    hash_tree::AprioriHashTree,
};
/// Runs the Apriori Algorithm
#[derive(Debug)]
pub struct Apriori {
    /// Minimum support count
    min_support: u64,
}
impl Apriori {
    /// Constructor
    pub fn new(min_support: u64) -> Self {
        Apriori { min_support }
    }
    /// Runs the algorithm
    pub fn run(self, data: &TransactionSet) -> Vec<Candidates> {
        let mut v = Vec::new();
        // First gets the frequent items
        v.push(apriori_run_one(data, self.min_support));
        for i in 2.. {
            // Creates the next frequent itemsets based on the previous frequent itemsets.
            let prev = v.last().unwrap();
            let next = AprioriCandidates::new(prev.deref()).run(data, i, self.min_support);
            if next.is_empty() {
                break;
            }
            v.push(next);
        }
        v
    }
    /// Runs the algorithm
    pub fn run_fn(self, data: &TransactionSet, mut f: impl FnMut(&[usize])) {
        let mut prev = apriori_run_one(data, self.min_support);
        prev.iter().for_each(|v| f(v));
        for i in 2.. {
            // Creates the next frequent itemsets based on the previous frequent itemsets.
            let next = AprioriCandidates::new(prev.deref()).run(data, i, self.min_support);
            if next.is_empty() {
                break;
            }
            prev = next;
            prev.iter().for_each(|v| f(v));
        }
    }
}
/// The wrapper for AprioriCandidates
pub struct AprioriCandidates<'a>(&'a CandidateType);
/// Dereferences to the underlying struct
impl Deref for AprioriCandidates<'_> {
    type Target = CandidateType;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a> AprioriCandidates<'a> {
    /// Constructor
    pub fn new(v: &'a CandidateType) -> Self {
        Self(v)
    }
    /// A prune function for Apriori
    pub fn can_be_pruned(&self, v: &[usize]) -> bool {
        if v.len() < 3 {
            return false;
        }
        // The vector used to check subsets
        let mut arr: Vec<_> = v.iter().cloned().skip(1).collect();
        // Check the subset without the first element.
        if !self.contains(&arr) {
            return true;
        }
        // Checks all the subsets without the nth element up to the last 2
        for i in 0..(v.len() - 3) {
            arr[i] = v[i];
            if !self.contains(&arr) {
                return true;
            }
        }
        false
    }
    pub fn create_tree(&self) -> AprioriHashTree {
        let mut tree = AprioriHashTree::new();
        // Joins relevant frequent itemsets
        join(self.iter(), |v| {
            // Prunes
            if self.can_be_pruned(&v) {
                return;
            }
            tree.add(&v);
        });
        tree
    }
    /// Counts the dataset
    pub fn run_count(&self, data: &TransactionSet, i: usize) -> AprioriHashTree {
        assert!(i > 2);
        let mut tree = self.create_tree();
        for d in data.iter() {
            apriori_count(d, i, tree.deref_mut(), |_| {});
        }
        tree
    }
    /// Runs the algorithm
    pub fn run(&self, data: &TransactionSet, i: usize, min_sup: u64) -> Candidates {
        if i == 1 {
            return apriori_run_one(data, min_sup);
        } else if i == 2 {
            return apriori_run_two(data, min_sup);
        }
        // Counts the dataset and creates the frequent itemsets
        let tree = self.run_count(data, i);
        let mut set = Candidates::default();
        for (arr, n) in tree.iter() {
            if n >= min_sup {
                set.insert(arr.to_vec());
            }
        }
        set
    }
}
pub trait AprioriCounting {
    fn len(&self) -> usize;
    fn increment(&mut self, v: &[usize]) -> bool;
    fn for_each_mut(&mut self, f: impl FnMut(&[usize], &mut u64));
}
pub fn apriori_count(
    t: &[usize],
    i: usize,
    d: &mut impl AprioriCounting,
    mut f: impl FnMut(&[usize]),
) {
    // Loops through each transaction in the dataset
    // Skips any that are of too little length
    if t.len() < i {
        return;
    }
    // A heuristic value to determine which way to count
    let mut combinations =
        ((t.len() - i + 1).max(i + 1)..=t.len()).fold(1f64, |acc, x| acc * (x as f64));
    if combinations.is_finite() {
        combinations /= (2..(t.len() - i + 1).min(i + 1)).fold(1f64, |a, n| a * (n as f64));
    }
    if (d.len() as f64) * (t.len() as f64) > combinations * 13f64 {
        // If the number of itemsets to be counted is larger, then count via nested loops
        nested_loops(
            |v| {
                if d.increment(v) {
                    f(v);
                }
            },
            &t,
            i,
        );
    } else {
        // Otherwise count for each itemset
        d.for_each_mut(|v, n| {
            let mut iter = t.iter().cloned();
            'outer: for &a in v {
                while let Some(b) = iter.next() {
                    match a.cmp(&b) {
                        std::cmp::Ordering::Less => return,
                        std::cmp::Ordering::Equal => continue 'outer,
                        std::cmp::Ordering::Greater => continue,
                    }
                }
                return;
            }
            *n += 1;
            f(v);
        });
    }
}

/// Apriori pass 1
pub fn apriori_run_one(d: &TransactionSet, min_sup: u64) -> Candidates {
    let first = apriori_run_one_count(d);
    let mut v = Candidates::default();
    for (i, n) in first.into_iter().enumerate() {
        if n >= min_sup {
            v.insert(vec![i]);
        }
    }
    v
}
/// Apriori pass 1 with the counts
pub fn apriori_run_one_count(d: &TransactionSet) -> Vec<u64> {
    // Uses a 1D array
    let mut first = vec![0u64; d.num_items];
    for d in d.iter() {
        for &item in d {
            first[item] += 1;
        }
    }
    first
}
/// Apriori pass 2 with counts
pub fn apriori_run_two_count(d: &TransactionSet) -> Array2D<u64> {
    // Counts through a 2D array (implementation is 1D through upper triangle)
    let mut second = Array2D::new(d.num_items);
    for d in d.iter() {
        for i in 0..d.len() {
            for j in 0..i {
                second.increment(d[i], d[j]);
            }
        }
    }
    second
}
/// Apriori pass 2
pub fn apriori_run_two(d: &TransactionSet, min_sup: u64) -> Candidates {
    let second = apriori_run_two_count(d);
    let mut v = Candidates::default();
    for (r, c, count) in second.iter() {
        if count >= min_sup {
            v.insert(vec![c, r]);
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use datasets::transaction_set::TransactionSet;

    use crate::apriori::Apriori;

    #[test]
    fn test_candidates() {
        let example = TransactionSet::new(
            vec![
                vec![0, 1, 4],
                vec![1, 3],
                vec![1, 2],
                vec![0, 1, 3],
                vec![0, 2],
                vec![1, 2],
                vec![0, 2],
                vec![0, 1, 2, 4],
                vec![0, 1, 2],
            ],
            5,
        );
        let apriori = Apriori::new(2);
        let result = apriori.run(&example);
        assert!(result[1].contains(&vec![0, 1]));
        assert!(result[1].contains(&vec![0, 2]));
        assert!(result[1].contains(&vec![0, 4]));
        assert!(result[1].contains(&vec![1, 2]));
        assert!(result[1].contains(&vec![1, 3]));
        assert!(result[1].contains(&vec![1, 4]));
        assert_eq!(result[1].len(), 6);
        assert_eq!(result.len(), 3);
        assert_eq!(result[2].len(), 2);
    }
}

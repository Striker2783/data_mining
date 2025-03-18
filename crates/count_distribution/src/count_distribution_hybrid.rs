use std::{
    sync::{Arc, Mutex},
    thread,
};

use apriori::candidates::Candidates;
use datasets::transaction_set::TransactionSet;

use crate::process_hybrid::CDProcessHybrid;
/// The parallelized version of AprioriHybrid
pub struct CountDistributionHybrid<'a> {
    /// The dataset used
    data: &'a TransactionSet,
    /// Number of threads
    threads: usize,
    /// The frequent itemsets found
    candidates: Vec<Arc<Candidates>>,
    /// Minimum support count
    min_sup: u64,
    /// What pass to switch to AprioriTID
    switch: usize,
}
impl<'a> CountDistributionHybrid<'a> {
    /// Constructor
    pub fn new(data: &'a TransactionSet, threads: usize, min_sup: u64, switch: usize) -> Self {
        assert!(switch > 2);
        Self {
            data,
            threads,
            candidates: Vec::new(),
            min_sup,
            switch,
        }
    }
    /// Runs the algorithm
    pub fn run(mut self) -> Vec<Arc<Candidates>> {
        // Contains the thread data
        let sets = self.partitions();
        for n in 1.. {
            // Runs the counting for each thread
            let mut handlers = Vec::new();
            for p in sets.iter() {
                let p = Arc::clone(p);
                let c = if n == 1 {
                    Arc::new(Candidates::default())
                } else {
                    Arc::clone(self.candidates.last().unwrap())
                };
                let t = thread::spawn(move || {
                    let mut p = p.lock().unwrap();
                    p.run(n, &c)
                });
                handlers.push(t);
            }
            // Gets the results of each thread
            let mut result = Vec::new();
            for h in handlers {
                result.push(h.join().unwrap());
            }
            // Combines them into one map
            let (first, rest) = result.split_at_mut(1);
            for r in rest {
                for (k, v) in r {
                    match first[0].get_mut(k) {
                        Some(v2) => *v2 += *v,
                        None => {
                            first[0].insert(k.to_vec(), *v);
                        }
                    }
                }
            }
            // Create the frequent itemsets
            let mut set = Candidates::default();
            for (k, &v) in &result[0] {
                if v >= self.min_sup {
                    set.insert(k.clone());
                }
            }
            if set.is_empty() {
                break;
            }
            self.candidates.push(Arc::new(set));
        }
        self.candidates
    }
    /// Partitions the data into threads
    fn partitions(&self) -> Vec<Arc<Mutex<CDProcessHybrid>>> {
        let mut v = Vec::new();
        // Each thread gets a partition of the database and what pass to switch
        for thread in 0..self.threads {
            let count = self.data.len() / self.threads;
            let slice = if thread == self.threads - 1 {
                &self.data[(count * thread)..self.data.len()]
            } else {
                &self.data[(count * thread)..(count * (thread + 1))]
            };
            let ts = TransactionSet::new(slice.to_vec(), self.data.num_items);
            let process = CDProcessHybrid::new(ts, self.switch);
            v.push(Arc::new(Mutex::new(process)));
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use datasets::transaction_set::TransactionSet;

    use crate::count_distribution_hybrid::CountDistributionHybrid;

    #[test]
    fn test_overall() {
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
        let cd = CountDistributionHybrid::new(&example, 8, 2, 3);
        let cd = cd.run();
        assert!(cd[0].contains(&vec![0]));
        assert!(cd[0].contains(&vec![1]));
        assert!(cd[0].contains(&vec![2]));
        assert!(cd[0].contains(&vec![3]));
        assert!(cd[0].contains(&vec![4]));
        assert_eq!(cd[0].len(), 5);
        assert_eq!(cd[1].len(), 6);
        assert_eq!(cd[2].len(), 2);
    }
}

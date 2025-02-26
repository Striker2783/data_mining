use std::{collections::HashMap, sync::Arc, thread};

use apriori::candidates::CandidateType;
use datasets::transaction_set::TransactionSet;

use crate::proccess::CDProcess;

pub struct CountDistrubtion {
    data: Arc<TransactionSet>,
    threads: usize,
    candidates: Vec<Arc<CandidateType>>,
    min_sup: u64,
}

impl CountDistrubtion {
    pub fn new(data: Arc<TransactionSet>, threads: usize, min_sup: u64) -> Self {
        Self {
            data,
            threads,
            candidates: Vec::new(),
            min_sup,
        }
    }

    pub fn run(mut self) -> Vec<Arc<CandidateType>> {
        let mut partitions = self.partitions();
        
        todo!()
    }
    fn partitions(&self) -> Vec<Arc<TransactionSet>> {
        let mut v = Vec::new();
        for i in 0..self.threads {
            v.push(Arc::new(self.partition(i)));
        }
        v
    }
    fn partition(&self, thread: usize) -> TransactionSet {
        let count = self.data.len() / self.threads;
        let slice = if thread == self.threads - 1 {
            &self.data[(count * thread)..self.data.len()]
        } else {
            &self.data[(count * thread)..(count * (thread + 1))]
        };
        TransactionSet::new(slice.to_vec(), self.data.num_items)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use datasets::transaction_set::TransactionSet;

    use super::CountDistrubtion;

    #[test]
    fn test_overall() {
        let example = Arc::new(TransactionSet::new(
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
        ));
        let cd = CountDistrubtion::new(example, 8, 2);
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

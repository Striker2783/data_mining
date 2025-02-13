use std::{collections::HashMap, sync::Arc, thread};

use datasets::transaction_set::TransactionSet;

use crate::proccess::CDProcess;

pub struct CountDistrubtion {
    data: Arc<TransactionSet>,
    threads: usize,
    candidates: Vec<Arc<Vec<Vec<usize>>>>,
    min_sup: u64,
}

impl CountDistrubtion {
    pub fn new(data: Arc<TransactionSet>, threads: usize, min_sup: u64) -> Self {
        Self { data, threads, candidates: Vec::new(), min_sup }
    }
    
    pub fn run(mut self) -> Vec<Arc<Vec<Vec<usize>>>> {
        for i in 1.. {
            let mut handles = Vec::new();
            for thread in 0..self.threads {
                let data = Arc::clone(&self.data);
                let threads = self.threads;
                let prev = match self.candidates.last() {
                    Some(c) => Arc::clone(c),
                    None => Arc::new(Vec::new()),
                };
                let handle = thread::spawn(move || {
                    let (start, end) = Self::partition(&data, threads, thread);
                    let partition = &data.transactions[start..end];
                    let process = CDProcess::new(partition, &prev);
                    process.run(i)
                });
                handles.push(handle);
            }
            let mut results = HashMap::new();
            for handle in handles {
                let result = handle.join().unwrap();
                for (k, v) in result.into_iter() {
                    results.entry(k).and_modify(|n| *n += v).or_insert(v);
                }
            }
            if results.is_empty() {
                break;
            }
            self.candidates.push(Arc::new(
                results
                    .into_iter()
                    .filter_map(|(k, v)| if v >= self.min_sup { Some(k) } else { None })
                    .collect(),
            ));
        }
        self.candidates
    }
    fn partition(data: &Arc<TransactionSet>, threads: usize, thread: usize) -> (usize, usize) {
        let count = data.transactions.len() / threads;
        if thread == threads - 1 {
            (count * thread, data.transactions.len())
        } else {
            (count * thread, count * (thread + 1))
        }
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
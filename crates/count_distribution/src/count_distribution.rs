use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    thread,
};

use apriori::candidates::Candidates;
use datasets::transaction_set::TransactionSet;

use crate::process::CDProcess;

pub struct CountDistribution {
    data: Arc<TransactionSet>,
    threads: usize,
    candidates: Vec<Arc<Candidates>>,
    min_sup: u64,
}

impl CountDistribution {
    pub fn new(data: Arc<TransactionSet>, threads: usize, min_sup: u64) -> Self {
        Self {
            data,
            threads,
            candidates: Vec::new(),
            min_sup,
        }
    }

    pub fn run(mut self) -> Vec<Arc<Candidates>> {
        let partitions = self.partitions();
        self.run_one(&partitions);
        self.run_two(&partitions);
        for n in 3.. {
            let mut handles = Vec::new();
            for p in &partitions {
                let p = Arc::clone(p);
                let candidates = Arc::clone(&self.candidates[n - 2]);
                let handle = thread::spawn(move || {
                    let cd = CDProcess::new(p, candidates);
                    cd.run(n)
                });
                handles.push(handle);
            }
            let mut results = Vec::new();
            for h in handles {
                results.push(h.join().unwrap());
            }
            let mut map = HashMap::new();
            for tree in results {
                for (v, n) in tree.iter() {
                    match map.get_mut(v) {
                        Some(n2) => *n2 += n,
                        None => {
                            map.insert(v.to_vec(), n);
                        }
                    }
                }
            }
            let mut set = Candidates::default();
            for (k, v) in map {
                if v >= self.min_sup {
                    set.insert(k);
                }
            }
            if set.is_empty() {
                break;
            }
            self.candidates.push(Arc::new(set));
        }
        self.candidates
    }
    fn run_two(&mut self, p: &[Arc<TransactionSet>]) {
        let mut handles = Vec::new();
        for i in 0..self.threads {
            let p = Arc::clone(&p[i]);
            let handle = thread::spawn(move || {
                let cd = CDProcess::new(p, Arc::new(Candidates::default()));
                cd.run_two()
            });
            handles.push(handle);
        }
        let mut results = Vec::new();
        for h in handles {
            results.push(h.join().unwrap());
        }
        let p = results.split_at_mut(1);
        for i in 0..p.1.len() {
            p.0[0].add_assign(&p.1[i]);
        }
        let mut map = Candidates::default();
        for (r, c, v) in results[0].iter() {
            if v >= self.min_sup {
                map.insert(vec![c, r]);
            }
        }
        self.candidates.push(Arc::new(map));
    }
    fn run_one(&mut self, p: &[Arc<TransactionSet>]) {
        let mut handles = Vec::new();
        for i in 0..self.threads {
            let p = Arc::clone(&p[i]);
            let handle = thread::spawn(move || {
                let cd = CDProcess::new(p, Arc::new(Candidates::default()));
                cd.run_one()
            });
            handles.push(handle);
        }
        let mut results = Vec::new();
        for h in handles {
            results.push(h.join().unwrap());
        }
        for i in 1..results.len() {
            for j in 0..results[i].len() {
                results[0][j] += results[i][j];
            }
        }
        let mut set = HashSet::new();
        for i in 0..results[0].len() {
            if results[0][i] >= self.min_sup {
                set.insert(vec![i]);
            }
        }
        self.candidates.push(Arc::new(set.into()));
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

    use crate::count_distribution::CountDistribution;

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
        let cd = CountDistribution::new(example, 8, 2);
        let cd = cd.run();
        assert!(cd[0].contains(&vec![0]));
        assert!(cd[0].contains(&vec![1]));
        assert!(cd[0].contains(&vec![2]));
        assert!(cd[0].contains(&vec![3]));
        assert!(cd[0].contains(&vec![4]));
        println!("{:?}", cd[1]);
        assert_eq!(cd[0].len(), 5);
        assert_eq!(cd[1].len(), 6);
        assert_eq!(cd[2].len(), 2);
    }
}
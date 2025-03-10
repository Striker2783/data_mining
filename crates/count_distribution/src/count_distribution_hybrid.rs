use std::{
    sync::{Arc, Mutex},
    thread,
};

use apriori::candidates::Candidates;
use datasets::transaction_set::TransactionSet;

use crate::process_hybrid::CDProcessHybrid;

pub struct CountDistributionHybrid<'a> {
    data: &'a TransactionSet,
    threads: usize,
    candidates: Vec<Arc<Candidates>>,
    min_sup: u64,
    switch: usize,
}
impl<'a> CountDistributionHybrid<'a> {
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

    pub fn run(mut self) -> Vec<Arc<Candidates>> {
        let sets = self.partitions();
        for n in 1.. {
            let mut handlers = Vec::new();
            for p in sets.iter().cloned() {
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
            let mut result = Vec::new();
            for h in handlers {
                result.push(h.join().unwrap());
            }
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

    fn partitions(&self) -> Vec<Arc<Mutex<CDProcessHybrid>>> {
        let mut v = Vec::new();
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

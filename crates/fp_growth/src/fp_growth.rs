use datasets::transaction_set::TransactionSet;

use crate::fp_tree::FPTree;

pub struct FPGrowth {
    min_sup: u64,
    data: TransactionSet,
}

impl FPGrowth {
    pub fn new(min_sup: u64, data: TransactionSet) -> Self {
        Self { min_sup, data }
    }
    pub fn run(mut self) {
        let mut map = vec![0u64; self.data.num_items];
        for t in self.data.iter() {
            for &n in t {
                map[n] += 1;
            }
        }
        let mut first = Vec::new();
        for (i, &n) in map.iter().enumerate() {
            if n < self.min_sup {
                continue;
            }
            first.push((i, n));
        }
        first.sort_by(|a, b| b.1.cmp(&a.1));
        let first: Vec<_> = first.into_iter().map(|a| a.0).collect();
        let mut tree = FPTree::new();
        tree.insert_transaction(&first);

        for t in self.data.iter_mut() {
            t.sort_by(|&a, &b| map[b].cmp(&map[a]));
            while let Some(&n) = t.last() {
                let count = map[n];
                if count < self.min_sup {
                    t.pop();
                    continue;
                }
                break;
            }
            tree.insert_transaction(&t);
        }
    }
}

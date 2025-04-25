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
    pub fn run(mut self) -> Vec<Vec<usize>> {
        let mut map = vec![0u64; self.data.num_items];
        for t in self.data.iter() {
            for &n in t {
                map[n] += 1;
            }
        }
        let mut tree = FPTree::new(self.min_sup);

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
        tree.mine()
    }
}

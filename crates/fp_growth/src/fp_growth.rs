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
    fn create_fp_tree(&mut self) -> FPTree {
        let mut map = vec![0u64; self.data.num_items];
        for t in self.data.iter() {
            for &n in t {
                map[n] += 1;
            }
        }
        let mut tree = FPTree::new(self.min_sup);
        for t in self.data.iter_mut() {
            t.retain(|&x| map[x] >= self.min_sup);
            t.sort_by(|&a, &b| map[b].cmp(&map[a]));
            tree.insert_transaction(&t);
        }
        tree
    }
    pub fn run(mut self) -> Vec<Vec<usize>> {
        let mut tree = self.create_fp_tree();
        tree.mine()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_fptree() {
        let transactionset = TransactionSet::new(
            vec![
                vec![0, 2, 3],    // T1: I1, I3, I4
                vec![1, 2, 4, 5], // T2: I2, I3, I5, I6
                vec![0, 1, 2, 4], // T3: I1, I2, I3, I5
                vec![1, 4],       // T4: I2, I5
                vec![0, 2, 4],    // T5: I1, I3, I5
            ],
            10,
        );
        let mut growth = FPGrowth::new(1, transactionset);
        let tree = growth.create_fp_tree();
        assert_eq!(tree.get(&[2]), 4); // {I3:4}
        assert_eq!(tree.get(&[4]), 1); // {I5:1}
        assert_eq!(tree.get(&[2, 0]), 1); // {I3->I1:1}
        assert_eq!(tree.get(&[2, 4]), 3); // {I3->I5:3}
        assert_eq!(tree.get(&[2, 0, 3]), 1); // {I3->I1->I4:1}
        assert_eq!(tree.get(&[2, 4, 1]), 1); // {I3->I5->I2:1}
        assert_eq!(tree.get(&[2, 4, 0]), 2); // {I3->I5->I1:1}
        assert_eq!(tree.get(&[4, 1]), 1); // {I5->I2:1}
        assert_eq!(tree.get(&[2, 4, 0, 1]), 1); // {I3->I5->I1->I2:2}
        assert_eq!(tree.get(&[2, 4, 1, 5]), 1); // {I3->I5->I2->I6:1}
    }
}

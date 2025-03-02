use datasets::transaction_set::TransactionSet;

use crate::trie::AprioriTrie;
#[derive(Debug)]
pub struct AprioriT {
    min_sup: u64,
    trie: AprioriTrie,
}

impl AprioriT {
    pub fn new(min_sup: u64) -> Self {
        Self { min_sup, trie: AprioriTrie::new() }
    }
    pub fn run(&mut self, t: &TransactionSet) {
        for i in 0..t.num_items {
            self.trie.add(&[i]);
        }
        for v in t.iter() {
            self.trie.transaction_update(v, 1);
        }
        for i in 2.. {
            let prev = self.trie.size();
            self.trie.cleaup(self.min_sup);
            self.trie.join(i, self.min_sup);
            if self.trie.size() <= prev {
                break;
            }
            for v in t.iter() {
                self.trie.transaction_update(v, i);
            }
        }
    }
    
    pub fn trie(self) -> AprioriTrie {
        self.trie
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use datasets::transaction_set::TransactionSet;

    use super::AprioriT;

    #[test]
    fn test_apriorit() {
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
        let mut a = AprioriT::new(2);
        a.run(&example);
        let mut set = HashSet::new();
        a.trie.for_each(2, |v| {
            set.insert(v.to_vec());
        });
        assert!(set.contains(&vec![0, 1]));
        assert!(set.contains(&vec![0, 2]));
        assert!(set.contains(&vec![0, 4]));
        assert!(set.contains(&vec![1, 2]));
        assert!(set.contains(&vec![1, 3]));
        assert!(set.contains(&vec![1, 4]));
        assert_eq!(set.len(), 13);
    }
}
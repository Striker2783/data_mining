use apriori::{apriori::apriori_run_one, trie::AprioriTrie};
use datasets::{transaction_set::TransactionSet, utils::nested_loops};

use crate::{frequent::Frequent, tree::Trie};

pub struct MaxMiner {
    min_sup: u64,
    data: TransactionSet,
}

impl MaxMiner {
    pub fn new(min_sup: u64, data: TransactionSet) -> Self {
        Self { min_sup, data }
    }
    pub fn run(self, mut f: impl FnMut(&[usize])) {
        let c = apriori_run_one(&self.data, self.min_sup);
        let mut v: Vec<_> = c.iter().map(|v| v[0]).collect();
        v.sort_unstable();
        let mut trie = Trie::new();
        trie.initial_groups(&v);
        let mut frequent = Frequent::new();
        frequent.add(&[*v.last().unwrap()]);
        for i in 1.. {
            for s in self.data.iter() {
                trie.count(s, i);
            }
            let mut count = 0;
            trie.count_frequent(
                i,
                |v| {
                    count += 1;
                    frequent.add(v);
                },
                self.min_sup,
            );
            let mut to_remove = Frequent::new();
            frequent.for_each(|v| {
                for i in 1..v.len() {
                    nested_loops(
                        |v| {
                            to_remove.add(v);
                        },
                        v,
                        i,
                    );
                }
            });
            trie.tails_filter(|v| {
                !to_remove.contains(v)
            }, i + 1);
            to_remove.for_each(|v| {
                frequent.remove(v);
            });
            if count == 0 {
                break;
            }
        }
        frequent.for_each(|v| {
            f(v);
        });
    }
}

use std::{fs::File, path::PathBuf};

use apriori::apriori_trie::AprioriT;
use clap::Args;
use datasets::transaction_set::TransactionSet;

#[derive(Args)]
pub struct AprioriTrieArgs {
    path: PathBuf,
    support_count: u64,
}
impl AprioriTrieArgs {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let f = File::open(&self.path)?;
        let data = TransactionSet::from_dat(f);
        let mut result = AprioriT::new(self.support_count);
        result.run(&data);
        let trie = result.trie();
        trie.for_each(self.support_count, |v| {
            for e in v {
                print!("{e} ");
            }
            println!();
        });
        Ok(())
    }
}
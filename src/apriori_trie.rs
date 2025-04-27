use std::{fs::File, path::PathBuf};

use apriori::apriori_trie::AprioriT;
use clap::Args;
use datasets::transaction_set::TransactionSet;

use crate::{get_writer, out_writer, Arguments};

#[derive(Args)]
pub struct AprioriTrieArgs {
    path: PathBuf,
    support_count: u64,
}
impl AprioriTrieArgs {
    pub fn run(&self, a: &Arguments) -> Result<(), Box<dyn std::error::Error>> {
        let f = File::open(&self.path)?;
        let data = TransactionSet::from_dat(f);
        let mut result = AprioriT::new(self.support_count);
        result.run(&data);
        let trie = result.trie();
        let mut out = get_writer(&a.output_file);
        trie.for_each(self.support_count, |v| {
            out_writer(v, &mut out);
        });
        Ok(())
    }
}
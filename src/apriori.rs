use std::{fs::File, path::PathBuf};

use apriori::apriori::Apriori;
use clap::Args;
use datasets::transaction_set::TransactionSet;

use crate::print_candidate;

#[derive(Args)]
pub struct AprioriArgs {
    path: PathBuf,
    support_count: u64,
}
impl AprioriArgs {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let f = File::open(&self.path)?;
        let data = TransactionSet::from_dat(f);
        let result = Apriori::new(self.support_count).run(&data);
        for c in result {
            print_candidate(c.data_owned().iter());
        }
        Ok(())
    }
}
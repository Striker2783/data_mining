use std::{error::Error, fs::File, path::PathBuf};

use apriori::apriori_hybrid::AprioriHybrid;
use clap::Args;
use datasets::transaction_set::TransactionSet;

use crate::print_candidate;

#[derive(Args)]
pub struct AprioriHybridArgs {
    path: PathBuf,
    support_count: u64,
    #[arg(default_value_t = 3)]
    switch: usize,
}

impl AprioriHybridArgs {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let t = TransactionSet::from_dat(File::open(&self.path)?);
        let c = AprioriHybrid::new(self.support_count, self.switch).run(&t);
        for c in c {
            print_candidate(c.iter());
        }
        Ok(())
    }
}

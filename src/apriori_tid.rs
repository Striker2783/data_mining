use std::{fs::File, path::PathBuf};

use apriori::apriori_tid::AprioriTID;
use clap::Args;
use datasets::transaction_set::TransactionSet;

use crate::print_candidate;

#[derive(Args)]
pub struct AprioriTIDArgs {
    path: PathBuf,
    support_count: u64,
}

impl AprioriTIDArgs {
    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let t = TransactionSet::from_dat(File::open(&self.path)?);
        let candidates = AprioriTID::new(self.support_count).run(&t);
        for c in candidates {
            print_candidate(c.candidates_owned().iter());
        }
        Ok(())
    }
}
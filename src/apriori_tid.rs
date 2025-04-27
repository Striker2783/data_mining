use std::{fs::File, path::PathBuf};

use apriori::apriori_tid::AprioriTID;
use clap::Args;
use datasets::transaction_set::TransactionSet;

use crate::{get_writer, out_writer, Arguments};

#[derive(Args)]
pub struct AprioriTIDArgs {
    path: PathBuf,
    support_count: u64,
}

impl AprioriTIDArgs {
    pub fn run(&self, a: &Arguments) -> Result<(), Box<dyn std::error::Error>> {
        let t = TransactionSet::from_dat(File::open(&self.path)?);
        let mut out = get_writer(&a.output_file);
        AprioriTID::new(self.support_count).run_fn(&t, |v| out_writer(v, &mut out));
        Ok(())
    }
}
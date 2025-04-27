use std::{error::Error, fs::File, path::PathBuf};

use apriori::apriori_hybrid::AprioriHybrid;
use clap::Args;
use datasets::transaction_set::TransactionSet;

use crate::{get_writer, out_writer, Arguments};

#[derive(Args)]
pub struct AprioriHybridArgs {
    path: PathBuf,
    support_count: u64,
    #[arg(default_value_t = 3)]
    switch: usize,
}

impl AprioriHybridArgs {
    pub fn run(&self, a: &Arguments) -> Result<(), Box<dyn Error>> {
        let t = TransactionSet::from_dat(File::open(&self.path)?);
        let mut out = get_writer(&a.output_file);
        AprioriHybrid::new(self.support_count, self.switch)
            .run_fn(&t, |v| out_writer(v, &mut out));
        Ok(())
    }
}

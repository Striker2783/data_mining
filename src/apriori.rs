use std::{fs::File, path::PathBuf};

use apriori::apriori::Apriori;
use clap::Args;
use datasets::transaction_set::TransactionSet;

use crate::{get_writer, out_writer, Arguments};

#[derive(Args)]
pub struct AprioriArgs {
    path: PathBuf,
    support_count: u64,
}
impl AprioriArgs {
    pub fn run(&self, config: &Arguments) -> Result<(), Box<dyn std::error::Error>> {
        let mut out = get_writer(&config.output_file);

        let f = File::open(&self.path)?;
        let data = TransactionSet::from_dat(f);
        Apriori::new(self.support_count).run_fn(&data, |v| out_writer(v, &mut out));
        let _ = out.flush();
        Ok(())
    }
}

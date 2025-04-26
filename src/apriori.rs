use std::{fs::File, path::PathBuf};

use apriori::apriori::Apriori;
use clap::Args;
use datasets::transaction_set::TransactionSet;

use crate::{get_writer, print_candidate, Arguments};

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
        let result = Apriori::new(self.support_count).run(&data);
        for c in result {
            print_candidate(c.iter());
        }
        Ok(())
    }
}
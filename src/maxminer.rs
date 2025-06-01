use std::{error::Error, fs::File, path::PathBuf};

use clap::Args;
use datasets::transaction_set::TransactionSet;
use maxminer::max_miner::MaxMiner;

use crate::{Arguments, get_writer, out_writer};

#[derive(Args)]
pub struct MaxMinerArgs {
    path: PathBuf,
    support_count: u64,
}
impl MaxMinerArgs {
    pub fn run(&self, a: &Arguments) -> Result<(), Box<dyn Error>> {
        let f = File::open(&self.path)?;
        let data = TransactionSet::from_dat(f);
        let max_miner = MaxMiner::new(self.support_count, data);
        let mut out = get_writer(&a.output_file);
        max_miner.run(|v| out_writer(v, &mut out));
        Ok(())
    }
}

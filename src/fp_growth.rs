use std::{error::Error, fs::File, path::PathBuf};

use clap::Args;
use datasets::transaction_set::TransactionSet;
use fp_growth::fp_growth::FPGrowth;

#[derive(Args)]
pub struct FPGrowthArgs {
    path: PathBuf,
    support_count: u64,
}
impl FPGrowthArgs {
    pub fn run(&self) -> Result<(), Box<dyn Error>>{
        let f = File::open(&self.path)?;
        let data = TransactionSet::from_dat(f);
        let fp_growth = FPGrowth::new(self.support_count, data);
        fp_growth.run();
        Ok(())
    }
}
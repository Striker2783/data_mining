use std::{error::Error, fs::File, path::PathBuf};

use clap::Args;
use datasets::transaction_set::TransactionSet;
use fp_growth::fp_growth::FPGrowth;

use crate::{Arguments, get_writer, out_writer};

#[derive(Args)]
pub struct FPGrowthArgs {
    path: PathBuf,
    support_count: u64,
}
impl FPGrowthArgs {
    pub fn run(&self, a: &Arguments) -> Result<(), Box<dyn Error>> {
        let f = File::open(&self.path)?;
        let data = TransactionSet::from_dat(f);
        let fp_growth = FPGrowth::new(self.support_count, data);
        let mut out = get_writer(&a.output_file);
        fp_growth.run_fn(|v| out_writer(v, &mut out));
        Ok(())
    }
}

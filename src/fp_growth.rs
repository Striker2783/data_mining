use std::{error::Error, fs::File, path::PathBuf};

use clap::Args;
use datasets::transaction_set::TransactionSet;
use fp_growth::fp_growth::FPGrowth;

use crate::print_candidate;

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
        let sets=  fp_growth.run();
        for mut v in sets {
            v.sort();
            for e in v {
                print!("{e} ");
            }
            println!()
        }
        Ok(())
    }
}
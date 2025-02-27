use std::{error::Error, fs::File, path::PathBuf, sync::Arc};

use clap::Args;
use count_distribution::{count_distribution::CountDistrubtion, count_distribution2::CountDistribution};
use datasets::transaction_set::TransactionSet;

use crate::print_candidate;

#[derive(Args)]
pub struct CountDistributionArgs {
    path: PathBuf,
    support_count: u64,
    #[arg(default_value_t = 1)]
    threads: usize,
}

impl CountDistributionArgs {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let t = TransactionSet::from_dat(File::open(&self.path)?);
        let candidates = CountDistrubtion::new(Arc::new(t), self.threads, self.support_count);
        let c = candidates.run();
        for c in c {
            print_candidate(c.iter());
        }
        Ok(())
    }
}

#[derive(Args)]
pub struct CountDistributionArgs2 {
    path: PathBuf,
    support_count: u64,
    #[arg(default_value_t = 1)]
    threads: usize,
}
impl CountDistributionArgs2 {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let t = TransactionSet::from_dat(File::open(&self.path)?);
        let candidates = CountDistribution::new(Arc::new(t), self.threads, self.support_count);
        let c = candidates.run();
        for c in c {
            print_candidate(c.iter());
        }
        Ok(())
    }
}
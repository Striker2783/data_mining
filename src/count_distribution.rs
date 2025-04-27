use std::{error::Error, fs::File, path::PathBuf, sync::Arc};

use clap::Args;
use count_distribution::count_distribution::CountDistribution;
use datasets::transaction_set::TransactionSet;

use crate::{get_writer, out_writer, Arguments};

#[derive(Args)]
pub struct CountDistributionArgs {
    path: PathBuf,
    support_count: u64,
    #[arg(default_value_t = 1)]
    threads: usize,
}

impl CountDistributionArgs {
    pub fn run(&self, a: &Arguments) -> Result<(), Box<dyn Error>> {
        let t = TransactionSet::from_dat(File::open(&self.path)?);
        let candidates = CountDistribution::new(Arc::new(t), self.threads, self.support_count);
        let c = candidates.run();
        let mut out = get_writer(&a.output_file);
        for c in c {
            for c in c.iter() {
                out_writer(c, &mut out);
            }
        }
        Ok(())
    }
}
use std::{fs::File, path::PathBuf};

use apriori::apriori_tid::AprioriTID;
use clap::Args;
use datasets::transaction_set::TransactionSet;

use crate::{get_writer, Arguments};

#[derive(Args)]
pub struct AprioriTIDArgs {
    path: PathBuf,
    support_count: u64,
}

impl AprioriTIDArgs {
    pub fn run(&self, a: &Arguments) -> Result<(), Box<dyn std::error::Error>> {
        let t = TransactionSet::from_dat(File::open(&self.path)?);
        let mut out = get_writer(&a.output_file);
        let closure = |v: &[usize]| {
            let mut string = String::new();
            for &e in v {
                string += format!("{e} ").as_str();
            }
            string += "\n";
            let _ = out.write(string.as_bytes());
        };
        AprioriTID::new(self.support_count).run_fn(&t, closure);
        Ok(())
    }
}
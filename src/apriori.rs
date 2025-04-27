use std::{fs::File, path::PathBuf};

use apriori::apriori::Apriori;
use clap::Args;
use datasets::transaction_set::TransactionSet;

use crate::{Arguments, get_writer};

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
        let closure = |v: &[usize]| {
            let mut string = String::new();
            for &e in v {
                string += format!("{e} ").as_str();
            }
            string += "\n";
            let _ = out.write(string.as_bytes());
        };
        Apriori::new(self.support_count).run_fn(&data, closure);
        let _ = out.flush();
        Ok(())
    }
}

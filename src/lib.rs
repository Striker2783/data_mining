mod apriori;
mod apriori_tid;

use std::error::Error;

use apriori::AprioriArgs;
use apriori_tid::AprioriTIDArgs;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Apriori(AprioriArgs),
    AprioriTID(AprioriTIDArgs)
}
impl Commands {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Commands::Apriori(apriori_args) => apriori_args.run()?,
            Commands::AprioriTID(apriori_args) => apriori_args.run()?,
        };
        Ok(())
    }
}

pub fn print_candidate<'a, T: Iterator<Item = &'a Vec<usize>>>(v: T) {
    for v in v {
        for &e in v {
            print!("{e} ");
        }
        println!()
    }
}
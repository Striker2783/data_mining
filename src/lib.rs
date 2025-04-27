mod apriori;
mod apriori_hybrid;
mod apriori_tid;
mod apriori_trie;
mod count_distribution;
mod count_distribution_hybrid;
pub mod fp_growth;

use std::{
    error::Error, fs::{self, File}, io::{self, BufWriter}, path::PathBuf
};

use apriori::AprioriArgs;
use apriori_hybrid::AprioriHybridArgs;
use apriori_tid::AprioriTIDArgs;
use apriori_trie::AprioriTrieArgs;
use clap::{Parser, Subcommand};
use count_distribution::CountDistributionArgs;
use count_distribution_hybrid::CountDistributionHybridArgs;
use fp_growth::FPGrowthArgs;


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Commands,
    /// Whether to print the amount of time the algoritm takes.
    #[arg(short, long, default_value_t = false)]
    pub time: bool,
    #[arg(short, long, global = true)]
    pub output_file: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum Commands {
    Apriori(AprioriArgs),
    AprioriTID(AprioriTIDArgs),
    AprioriHybrid(AprioriHybridArgs),
    CountDistribution(CountDistributionArgs),
    AprioriTrie(AprioriTrieArgs),
    CountDistributionHybrid(CountDistributionHybridArgs),
    FPGrowth(FPGrowthArgs),
}
impl Commands {
    pub fn run(&self, a: &Arguments) -> Result<(), Box<dyn Error>> {
        match self {
            Commands::Apriori(apriori_args) => apriori_args.run(a)?,
            Commands::AprioriTID(apriori_args) => apriori_args.run()?,
            Commands::AprioriHybrid(apriori_hybrid_args) => apriori_hybrid_args.run()?,
            Commands::CountDistribution(count_distribution_args) => {
                count_distribution_args.run()?
            }
            Commands::AprioriTrie(apriori_trie_args) => apriori_trie_args.run()?,
            Commands::CountDistributionHybrid(count_distribution_hybrid_args) => {
                count_distribution_hybrid_args.run()?
            }
            Commands::FPGrowth(fpgrowth_args) => fpgrowth_args.run()?,
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

pub fn get_writer(path: &Option<PathBuf>) -> Box<dyn io::Write> {
    if let Some(path) = path {
        let file = File::create(path).unwrap();
        return Box::new(BufWriter::new(file));
    } else {
        return Box::new(io::stdout().lock());
    }
}

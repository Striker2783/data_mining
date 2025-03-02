mod apriori;
mod apriori_hybrid;
mod apriori_tid;
mod apriori_trie;
mod count_distribution;

use std::error::Error;

use apriori::AprioriArgs;
use apriori_hybrid::AprioriHybridArgs;
use apriori_tid::AprioriTIDArgs;
use apriori_trie::AprioriTrieArgs;
use clap::Subcommand;
use count_distribution::CountDistributionArgs;

#[derive(Subcommand)]
pub enum Commands {
    Apriori(AprioriArgs),
    AprioriTID(AprioriTIDArgs),
    AprioriHybrid(AprioriHybridArgs),
    CountDistribution(CountDistributionArgs),
    AprioriTrie(AprioriTrieArgs),
}
impl Commands {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Commands::Apriori(apriori_args) => apriori_args.run()?,
            Commands::AprioriTID(apriori_args) => apriori_args.run()?,
            Commands::AprioriHybrid(apriori_hybrid_args) => apriori_hybrid_args.run()?,
            Commands::CountDistribution(count_distribution_args) => {
                count_distribution_args.run()?
            }
            Commands::AprioriTrie(apriori_trie_args) => apriori_trie_args.run()?,
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

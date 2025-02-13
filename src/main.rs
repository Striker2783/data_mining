use std::{fs::File, path::PathBuf, sync::Arc};

use apriori::apriori::Apriori;
use clap::{Parser, ValueEnum};
use count_distribution::count_distribution::CountDistrubtion;
use datasets::transaction_set::TransactionSet;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(short, long, default_value = "dat")]
    file_type: String,
    #[arg(short, long, required = true)]
    path: PathBuf,
    #[arg(short, long, required = true)]
    algorithm: Algorithms,
}
#[derive(Debug, Clone, Copy, ValueEnum)]
enum Algorithms {
    Apriori,
    CountDistribution,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();
    let f = File::open(&args.path)?;
    let t = match args.file_type.as_str() {
        "dat" => TransactionSet::from_dat(f),
        _ => return Err("Unsupported file type".into()),
    };
    match args.algorithm {
        Algorithms::Apriori => {
            let apriori = Apriori::new(&t, (t.transactions.len() / 100) as u64, 2);
            let candidates = apriori.run();
            println!("{:?}", candidates.candidates());
        },
        Algorithms::CountDistribution => {
            let t = Arc::new(t);
            let min_sup = (t.transactions.len() / 100) as u64;
            let count_distribution = CountDistrubtion::new(t, 8, min_sup);
            let candidates = count_distribution.run();
            println!("{:?}", candidates);
        },
    };
    Ok(())
}

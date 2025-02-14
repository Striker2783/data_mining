use std::time::Instant;

use clap::Parser;
use data_mining::Commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long, default_value_t = false)]
    time: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();

    if args.time {
        let start = Instant::now();
        args.command.run()?;
        println!("Time taken: {:?}", start.elapsed());
    } else {
        args.command.run()?;
    }

    Ok(())
}

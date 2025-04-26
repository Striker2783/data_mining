use std::
    time::Instant
;

use clap::Parser;
use data_mining::Arguments;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse();

    if args.time {
        let start = Instant::now();
        args.command.run(&args)?;
        println!("Time taken: {:?}", start.elapsed());
    } else {
        args.command.run(&args)?;
    }

    Ok(())
}

use clap::Parser;

mod args;
mod converter;

use args::Args;
use args::Commands;

fn main() {
    let args = Args::parse();
    let string = args.string;

    match &args.command {
        Some(Commands::Raw) => {
            let raw = converter::get_raw(string.to_owned());
            println!("{} = {:?}", string, raw);
        }
        Some(Commands::Sum) => {
            let sum = converter::get_sum(string.to_owned());
            println!("{} = {}", string, sum);
        }
        None => println!("No Arguments"),
    }
}

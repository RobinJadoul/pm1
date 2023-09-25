use clap::Parser;
use pm1::{pm1, Verbosity};
use rug::Integer as Int;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(long)]
    logb1: u64,
    #[arg(long, default_value_t = 1)]
    exp: u64,
    #[arg(short, long)]
    logb2: Option<u64>,
    #[arg(short)]
    n: String,
    #[arg(short)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    let logb2 = args.logb2.unwrap_or(args.logb1);
    let verbosity = if args.verbose {
        Verbosity::Tqdm
    } else {
        Verbosity::Silent
    };
    println!(
        "{:?}",
        pm1(
            args.logb1,
            args.exp,
            logb2,
            &Int::from_str_radix(&args.n, 10).unwrap(),
            verbosity
        )
    );
}

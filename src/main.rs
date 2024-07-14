mod fibonacci;

use clap::Parser;
use fibonacci::calculate;

#[derive(Parser)]
/// Calculate fibonacci number with 64-bit wrapping
struct Args {
    #[arg(short)]
    /// n-th fibonacci
    n: usize,
}

fn main() {
    let args: Args = Args::parse();

    println!("{}", calculate(args.n));
}

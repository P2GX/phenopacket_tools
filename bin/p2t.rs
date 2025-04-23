

use clap::Parser;
use std::path::Path;


/// A simple CLI example
#[derive(Parser)]
#[command(name = "p2t")]
#[command(about = "Phenopacket tools CLI", long_about = None)]
struct Cli {
  /*  /// A required input file
    #[arg(short, long)]
    template: String,

    #[arg(short, long)]
    json: String,
 */
    /// An optional flag
    #[arg(short, long)]
    verbose: bool,
}



fn main() {
    let cli = Cli::parse();
    println!("hello")
}

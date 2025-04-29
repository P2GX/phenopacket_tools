

use clap::Parser;
use std::path::Path;
mod bethlam_myopathy;
use crate::bethlam_myopathy::bethlem_myopathy_phenopacket;




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
    let myopathy = bethlem_myopathy_phenopacket();
    println!("{:?}", &myopathy);
    let json = serde_json::to_string_pretty(&myopathy).unwrap();
    println!("{}", json);
    let yaml = serde_yaml::to_string(&myopathyq).unwrap();
    println!("{}", yaml);
}

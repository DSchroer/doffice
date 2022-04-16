extern crate core;

mod calc;
mod doc;

use std::error::Error;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version)]
#[clap(author = "Dominick Schroer <dominick@schroer.ca>")]
#[clap(about = "Plain text office suite", long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Process CSV file
    Calc {
        file: String
    },
    /// Process markdown document
    Doc {
        file: String
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        Commands::Calc { file } => calc::evaluate_csv_file(file),
        Commands::Doc { file } => doc::process_markdown_file(file),
    }
}

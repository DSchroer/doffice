extern crate core;

mod calc;
mod doc;
mod show;

use std::error::Error;
use clap::{Parser, Subcommand};
use crate::show::Theme;

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
    /// Create slides from markdown
    Show {
        file: String,
        /// Use dark theme base
        #[clap(short, long)]
        dark: bool,
        /// CSS theme file to apply to slides
        #[clap(short, long)]
        theme: Option<String>
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        Commands::Calc { file } => calc::evaluate_csv_file(file),
        Commands::Doc { file } => doc::process_markdown_file(file),
        Commands::Show { file, dark, theme } => {
            let base_theme = if dark { Theme::Black } else { Theme::White };
            show::slides_from_file(file, base_theme, theme)
        },
    }
}

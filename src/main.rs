extern crate core;

mod calc;
mod doc;
mod show;
mod framework;

use std::error::Error;
use std::path::Path;
use clap::{Parser, Subcommand};
use crate::calc::Source;
use crate::framework::RunnerConfig;
use crate::show::Theme;

#[derive(Parser)]
#[clap(author, version)]
#[clap(author = "Dominick Schroer <dominick@schroer.ca>")]
#[clap(about = "Plain text office suite", long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
    /// Watch mode
    #[clap(short, long)]
    watch: bool
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
        Commands::Calc { file } => {
            let config = config(args.watch, &file, "html");
            calc::evaluate_csv_file(Source::FromFile(file), config)
        },
        Commands::Doc { file } => {
            let config = config(args.watch, &file, "html");
            doc::process_markdown_file(file, config)
        },
        Commands::Show { file, dark, theme } => {
            let config = config(args.watch, &file, "html");
            let base_theme = if dark { Theme::Black } else { Theme::White };
            show::slides_from_file(file, base_theme, theme, config)
        },
    }
}

fn config<'a>(watch: bool, file: &str, ext: &str) -> RunnerConfig<'a>{
    let outfile = &Path::new(file).with_extension(format!("out.{}", ext));
    if watch {
        RunnerConfig::Watch(8080, String::from(file))
    } else {
        RunnerConfig::ToFile(String::from(outfile.to_str().unwrap()))
    }
}

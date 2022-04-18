extern crate core;

mod calc;
mod doc;
mod show;
mod framework;

use std::error::Error;
use std::path::Path;
use clap::{Parser, Subcommand};
use crate::calc::{Formatter, Source};
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
        file: String,
        /// CSS theme file to apply to tables
        #[clap(short, long)]
        theme: Option<String>
    },
    /// Process markdown document
    Doc {
        file: String,
        /// CSS theme file to apply to the document
        #[clap(short, long)]
        theme: Option<String>
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
        Commands::Calc { file, theme } => {
            let config = config(args.watch, &file, "html", &theme);
            let format = Formatter::Html { watch: args.watch, theme_file: theme };
            calc::evaluate_csv_file(Source::FromFile(file), format, config)
        },
        Commands::Doc { file, theme } => {
            let config = config(args.watch, &file, "html", &theme);
            doc::process_markdown_file(file, theme, config)
        },
        Commands::Show { file, dark, theme } => {
            let config = config(args.watch, &file, "html", &theme);
            let base_theme = if dark { Theme::Black } else { Theme::White };
            show::slides_from_file(file, base_theme, theme, config)
        },
    }
}

fn config<'a>(watch: bool, file: &str, ext: &str, theme: &Option<String>) -> RunnerConfig<'a>{
    let outfile = &Path::new(file).with_extension(format!("out.{}", ext));
    if watch {
        let mut paths = vec![String::from(file)];
        if let Some(theme_path) = theme {
            paths.push(theme_path.clone());
        }
        RunnerConfig::Watch(8080, paths)
    } else {
        RunnerConfig::ToFile(String::from(outfile.to_str().unwrap()))
    }
}

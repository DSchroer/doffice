extern crate core;

mod calc;
mod doc;
mod show;
mod framework;
mod html;

use std::error::Error;
use std::path::{Path, PathBuf};
use clap::{Parser, Subcommand};
use crate::calc::{Calc};
use crate::doc::Doc;
use crate::framework::{Printer, Loader, print_to_file, print_to_web};
use crate::html::HtmlPrinter;
use crate::show::{Slides};

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
        /// CSS theme file to apply to slides
        #[clap(short, long)]
        theme: Option<String>
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match &args.command {
        Commands::Calc { file, theme } => {
            let printer = HtmlPrinter::new(args.watch, theme.clone());
            let calc = Calc::from_file(file.clone());
            run_command(&args, calc, printer)
        },
        Commands::Doc { file, theme } => {
            let printer = HtmlPrinter::new(args.watch, theme.clone());
            let doc = Doc::new(Path::new(&file));
            run_command(&args, doc, printer)
        },
        Commands::Show { file, theme } => {
            let printer = HtmlPrinter::new(args.watch, theme.clone());
            let slides = Slides::new(Path::new(&file));
            run_command(&args, slides, printer)
        },
    }
}

fn run_command<T>(args: &Args, loader: impl Loader<Result=T>, printer: impl Printer<T>) -> Result<(), Box<dyn Error>> {
    let watch_paths = watch_paths(&args.command);

    if args.watch {
        print_to_web(loader, printer, 8080, watch_paths)
    }else {
        let outfile = out_file(&args.command);
        print_to_file(loader, printer, &outfile)
    }
}

fn out_file(command: &Commands) -> PathBuf {
    match command {
        Commands::Calc { file, .. } => {
            Path::new(&file).with_extension("out.html")
        }
        Commands::Doc { file, .. } => {
            Path::new(&file).with_extension("out.html")
        }
        Commands::Show { file, .. } => {
            Path::new(&file).with_extension("out.html")
        }
    }
}

fn watch_paths(command: &Commands) -> Vec<String>{
    let mut paths = Vec::new();

    match command {
        Commands::Calc { file, theme, .. } => {
            paths.push(file.clone());
            if let Some(theme) = theme {
                paths.push(theme.clone())
            }
        }
        Commands::Doc { file, theme, .. } => {
            paths.push(file.clone());
            if let Some(theme) = theme {
                paths.push(theme.clone())
            }
        }
        Commands::Show { file, theme, .. } => {
            paths.push(file.clone());
            if let Some(theme) = theme {
                paths.push(theme.clone())
            }
        }
    };

    paths
}

#![allow(unused)]

use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use clap::{Parser, Subcommand, Args};
use keride::cesr::{Serder, Saider, Sadder};
use keride::dat;

mod commands;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate and embed Self Addressing IDentifer (SAID) in map content
    Saidify(SaidifyArgs),
}

#[derive(Args)]
struct SaidifyArgs {
    #[arg(short, long)]
    file: Option<PathBuf>,
}


fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Saidify(args) => {
            commands::saidify::saidify(args.file.as_ref().unwrap());
        }
    }
}

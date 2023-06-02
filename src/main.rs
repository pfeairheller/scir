#![allow(unused)]

use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use clap::{Parser, Subcommand, Args};
use keride::cesr::{Serder, Saider, Sadder};
use keride::dat;

mod commands;
mod core;
mod app;


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
    Connect(ConnectArgs),
}

#[derive(Args)]
struct SaidifyArgs {
    #[arg(short, long)]
    file: Option<PathBuf>,
}

#[derive(Args)]
struct ConnectArgs {
    #[arg(short, long)]
    url: Option<String>,
}


fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Saidify(args) => {
            commands::saidify::saidify(args.file.as_ref().unwrap());
        }
        Commands::Connect(args) => {
            commands::connect::connect(args.url.as_ref().unwrap());
        }
    }
}

#![allow(unused)]

use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use clap::{Parser, Subcommand, Args};
use keride::cesr::{Serder, Saider, Sadder};
use keride::dat;


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
            let path = args.file.as_ref().unwrap();
            let fs = File::open(path).unwrap();
                // .with_context(|| format!("could not read file `{:#?}`", &path.into_os_string()))?;
            let mut reader = BufReader::new(fs);
            let val: serde_json::Value  = serde_json::from_reader(reader).unwrap();

            let e1 = dat!(&val);
            let (_, mut e2) = Saider::saidify(&e1, None, None, None, None).unwrap();
            let serder = Serder::new(None, None, None, Some(&e2), None).unwrap();

            println!("{}", serder.pretty(Some(1000)).unwrap())
        }
    }
}

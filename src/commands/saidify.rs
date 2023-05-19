use anyhow::{Context, Result};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use keride::cesr::{Saider, Serder, Sadder};
use keride::dat;

pub (crate) fn saidify(path: &PathBuf) -> Result<()> {
    let fs = File::open(path).unwrap();
    // .with_context(|| format!("could not read file `{:#?}`", &path.into_os_string()))?;
    let mut reader = BufReader::new(fs);
    let val: serde_json::Value  = serde_json::from_reader(reader).unwrap();

    let e1 = dat!(&val);
    let (_, mut e2) = Saider::saidify(&e1, None, None, None, None).unwrap();
    let serder = Serder::new(None, None, None, Some(&e2), None).unwrap();

    println!("{}", serder.pretty(Some(1000)).unwrap());
    Ok(())
}
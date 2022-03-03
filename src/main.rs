use clap::Parser;
use std::boxed::Box;
// use url::{ParseError, Url};

use std::fs::File;
use std::io::prelude::*; // we need that for BufReader lines
use std::io::{BufReader, Error};

fn main() {
    let args = Cli::parse();
    let path = &args.wordlist;

    let wordlist = open_wordlist(path).unwrap();
    for line in wordlist {
        println!("{}", line);
    }
}

fn open_wordlist(path: &str) -> Result<Box<impl Iterator<Item = String>>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap_or_default());
    Ok(Box::new(reader))
}

#[derive(Parser)]
pub struct Cli {
    wordlist: String,
}

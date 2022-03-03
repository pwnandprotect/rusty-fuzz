use clap::Parser;
use std::boxed::Box;
// use url::{ParseError, Url};

use std::fs::File;
use std::io::prelude::*; // we need that for BufReader lines
use std::io::{BufReader, Error};
use std::net::Ipv4Addr;

fn main() {
    let args = Cli::parse();
    let path = &args.wordlist;

    let wordlist = open_wordlist(path).unwrap();
    for line in wordlist {
        println!("{}", line);
    }
}
#[derive(Parser)]
pub struct Cli {
    wordlist: String,
    #[clap(parse(from_os_str))]
    ip: Ipv4Addr,
}

/*
TODO:
- add IP to cli arguments
- make sync request to IP / url
- take response status code
- check if response status code is within the list of allowed status codes?
- create cli table
- make async via tokio
- add thread limits
- add extension cli parameter
- use extension parameter to fuzz for files
- define default wordlist
*/

fn open_wordlist(path: &str) -> Result<Box<impl Iterator<Item = String>>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap_or_default());
    Ok(Box::new(reader))
}

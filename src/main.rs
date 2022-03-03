use clap::Parser;
use std::boxed::Box;

use reqwest;

use std::fs::File;
use std::io::prelude::*; // we need that for BufReader lines
use std::io::{BufReader, Error};

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

fn main() {
    let args = Cli::parse();
    let path = &args.wordlist;
    let ip = &args.ip;
    let _connection = http_request(ip);
    let _wordlist = open_wordlist(path).unwrap();
}
#[derive(Parser)]
pub struct Cli {
    wordlist: String,
    ip: String,
}

fn open_wordlist(path: &str) -> Result<Box<impl Iterator<Item = String>>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap_or_default());
    Ok(Box::new(reader))
}

fn http_request(ip: &str) -> Result<(), Box<dyn std::error::Error>> {
    let uri = format!("http://{}", ip);
    println!("{}", uri);
    let resp = reqwest::blocking::get(uri)?;
    println!("{:#?}", resp.status());
    Ok(())
}

use clap::Parser;
use std::boxed::Box;

use reqwest::{self, StatusCode};
use std::fs::File;
use std::io::prelude::*; // we need that for BufReader lines
use std::io::{BufReader, Error};
//use url::Url;

use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use term_table::Table;
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
    let path = args.wordlist;
    let url = args.url;
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(r#" 
                                /$$                                    /$$$$$$                            
                               | $$                                   /$$__  $$                           
 /$$$$$$   /$$   /$$  /$$$$$$$/$$$$$$   /$$   /$$                    | $$  \__/$$   /$$ /$$$$$$$$/$$$$$$$$
/$$__  $$ | $$  | $$ /$$_____/_  $$_/  | $$  | $$       /$$$$$$      | $$$$  | $$  | $$|____ /$$/____ /$$/
| $$  \__/| $$  | $$|  $$$$$$  | $$    | $$  | $$      |______/      | $$_/  | $$  | $$   /$$$$/   /$$$$/ 
| $$      | $$  | $$ \____  $$ | $$ /$$| $$  | $$                    | $$    | $$  | $$  /$$__/   /$$__/  
| $$      |  $$$$$$/ /$$$$$$$/ |  $$$$/|  $$$$$$$                    | $$    |  $$$$$$/ /$$$$$$$$/$$$$$$$$
|__/       \______/ |_______/   \___/   \____  $$                    |__/     \______/ |________/________/
                                        /$$  | $$                                                         
                                       |  $$$$$$/                                                         
                                        \______/                                                           "#, 2, Alignment::Left)]));

    let wordlist = open_wordlist(&path).unwrap();
    for line in wordlist {
        let response = http_request(format!("{}{}", &url, line)).unwrap();
        match response.status() {
            StatusCode::NOT_FOUND => (),
            _ => table.add_row(Row::new(vec![
                TableCell::new_with_alignment(response.url(), 1, Alignment::Left),
                TableCell::new_with_alignment(response.status(), 1, Alignment::Right),
            ])),
        }
    }
    println!("{}", table.render());
}
#[derive(Parser)]
pub struct Cli {
    wordlist: String,
    #[clap(short, long)]
    url: String,
}

fn open_wordlist(path: &str) -> Result<Box<impl Iterator<Item = String>>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap_or_default());
    Ok(Box::new(reader))
}

fn http_request(url: String) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url)?;
    Ok(resp)
}

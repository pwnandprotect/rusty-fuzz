use clap::Parser;
use std::boxed::Box;

use reqwest::{self, StatusCode, Url};
use std::fs::File;
use std::io::prelude::*; // we need that for BufReader lines
use std::io::{BufReader, Error};
use std::time::Instant;

//use url::Url;

use term_table::row::Row;
use term_table::table_cell::{Alignment, TableCell};
use term_table::Table;

/*
TODO:
- check if response status code is within the list of allowed status codes (200,201,204,301,302,307,401,403)?
- add thread limits + cli parameter
- add extension cli parameter
- use extension parameter to fuzz for files
- define default wordlist?
    - package small wordlist with binary?
- optional recursive flag + implementation
- define timeout
- define custom user agent?
*/

/*
gobuster dir -u https://buffered.io -w ~/wordlists/shortlist.txt

===============================================================
Gobuster v3.1.0
by OJ Reeves (@TheColonial) & Christian Mehlmauer (@firefart)
===============================================================
[+] Mode         : dir
[+] Url/Domain   : https://buffered.io/
[+] Threads      : 10
[+] Wordlist     : /home/oj/wordlists/shortlist.txt
[+] Status codes : 200,204,301,302,307,401,403
[+] User Agent   : gobuster/3.1.0
[+] Timeout      : 10s
===============================================================
2019/06/21 11:49:43 Starting gobuster
===============================================================
/categories (Status: 301)
/contact (Status: 301)
/posts (Status: 301)
/index (Status: 200)
===============================================================
2019/06/21 11:49:44 Finished
===============================================================
 */

#[derive(Parser)]
pub struct Cli {
    url: Url,
    wordlist: String,
    // threads: u32,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let path = &args.wordlist;
    let url = &args.url;
    // let _threads = &args.threads;

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
                                            \______/                                                          "#, 2, Alignment::Center)]));

    let start = Instant::now();
    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        format!("start: {:?}", start),
        1,
        Alignment::Left,
    )]));

    let wordlist = open_wordlist(&path).unwrap();
    for line in wordlist {
        let response = http_request(&url.join(&line).unwrap()).await.unwrap();
        //println!("{}", response.status());
        match response.status() {
            StatusCode::NOT_FOUND => (),

            _ => table.add_row(Row::new(vec![
                TableCell::new_with_alignment(response.url(), 1, Alignment::Left),
                TableCell::new_with_alignment(response.status(), 1, Alignment::Right),
            ])),
            // _ => println!("{} - {}", response.url(), response.status()),
        }
    }
    let elapsed = start.elapsed();
    // println!("{:?}", elapsed);

    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        format!("duration: {:?}", elapsed),
        1,
        Alignment::Left,
    )]));
    println!("{}", table.render());
}

fn open_wordlist(path: &str) -> Result<Box<impl Iterator<Item = String>>, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap_or_default());
    Ok(Box::new(reader))
}

async fn http_request(base_url: &Url) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let resp = reqwest::get(base_url.as_ref()).await?;
    Ok(resp)
}

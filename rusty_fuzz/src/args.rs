use std::time::Duration;
use clap::{
    crate_version,
    App,
    Arg,
    ArgMatches,
    Values
};

use rusty_fuzz_lib::UrlIterator;

const DEFAULT_RETRY: u8 = 2; // default number of additional attempts to try to open a url
const DEFAULT_TIMEOUT_IN_MS: u64 = 1_000;
const DEFAULT_WORDLIST_PATH: String = "/usr/share/wordlists/SecLists/Discovery/Web-Content/directory-list-2.3-medium.txt";
const DEFAULT_RECURSIVE_MODE: bool = false; 

pub(crate) struct RustyFuzzConfig {
    pub(crate) url: UrlIterator,
    pub(crate) wordlist: Path,
    pub(crate) grep_mode: bool, // shall we call this quiet mode?
    pub(crate) timeout: Duration,
    pub(crate) retries: u8,
    pub(crate) extensions: Option<Vec<String>>,
    pub(crate) http_codes: Option<Vec<u16>>,
    pub(crate) recursive_mode: bool,
}

pub(crate) fn get_rusty_fuzz_cli_config() -> RustyFuzzConfig {
    let matches = app_config().get_matches();

    let url = get_url(&matches);
    let wordlist = get_wordlist(&matches);
    let grep_mode = get_grep_mode(&matches);
    let timeout = get_timeout(&matches);
    let retries = get_retries(&matches);
    let extensions = get_extensions(&matches);
    let http_codes = get_http_codes(&matches);
    let recursive_mode = get_recursive_mode(&matches);

    RustyFuzzConfig {
        url,
        wordlist,
        grep_mode,
        timeout,
        retries,
        extensions,
        http_codes,
        recursive_mode,
    }
}

fn get_url(matches: &ArgMatches) -> UrlIterator {
    matches
        .values_of("url")
        .expect("Url is required to run rusty-fuzz")
        // fold creates a single object after/while traversing through the given object via iterator
        // it basically saves the first closure argument as a temp. variable after each iteration
        // the 2nd closure argument is the variable that changes with each iteration
        .fold(UrlIterator::new(), |url_iterator, url_str| {
            if let Ok(url) = Url::from_str(url_str) {
                url_iterator.add_url(url)
            } else {
                // TODO what do we do if we cannot parse the url? request DNS record if an IP was provided?
                // TODO PARSE IP -> lookup etc/hosts? windows equivalent?

                //let url_from_ip = <PARSE_ME_BABY_ONE_MORE_TIME>;
                //url_iterator.add_url(url_from_ip)
            }
        })
}

fn get_wordlist(matches: &ArgMatches) -> Result<Box<impl Iterator<Item = String>>, Error> {
        let filename = matches.value_of("wordlist").map(|value| {
            value.parse::<Path>().expect(&format!("unable to parse FileName - {}", value).unwrap_or_default(DEFAULT_WORDLIST_PATH)
            }
        );
        let file = File::open(filename)?;
        let reader = BufReader::new(file)
            .lines()
            .map(|line| line.unwrap_or_default());
        Ok(Box::new(reader))
    }
    
}

fn get_grep_mode(matches: &ArgMatches) -> bool {
    matches.is_present("grep_mode")
}


fn get_timeout(matches: &ArgMatches) -> Duration {
    let timeout = matches.value_of("timeout").map(|value| {
        value
            .parse::<u64>()
            .expect(&format!("Unable to parse timeout value - {}", value))
        })
        .unwrap_or(DEFAULT_TIMEOUT_IN_MS);

        Duration::from_millis(timeout)
}

fn get_retries(matches: &ArgMatches) -> u8 {
    let retries = matches
                    .value_of("retries")
                    .map(|value| {
        value
            .parse::<u8>()
            .expect(&format!("Unable to parse retries value - {}", value))
    })
    .unwrap_or(DEFAULT_RETRY);
}

fn get_extensions(matches: &ArgMatches) -> <Option<ExtensionIterator>> {
    matches
    .values_of("extensions").map(|value| value.to_string()).collect()
}

fn get_http_codes() {
 // TODO
}

fn get_recursive_mode(matches: &ArgMatches) -> bool {
    matches.is_present("recursive_mode")
}

fn app_config() -> App<'static, 'static> {
    App::new("rusty-fuzz")
        .author("maikroservice <maikroservice@gmail.com>")
        .about("High performance forced directory browser")
        .version(crate_version!())
        .arg(Arg::with_name("url")
            .help("The base-url to scan.")
            .index(1)
            .multiple(false)
            .takes_value(true)
            .require_delimiter(false)
            .required(true)
        )
        .arg(Arg::with_name("wordlist")
            .help("the wordlist to use (one item per line)")
            .short("w")
            .long("wordlist")
            .takes_value(true)
            .required(false)
        )
        .arg(Arg::with_name("grep_mode")
            .help("output in a greppable format")
            .short("g")
            .takes_value(false)
            .required(false)
        )
        .arg(Arg::with_name("timeout")
            .help("The amount of time in milliseconds to wait for a connection to be counted as timed-out, defaults to 1000 ms. ")
            .short("t")
            .long("timeout")
            .takes_value(true)
        )
        .arg(Arg::with_name("retries")
            .help("The number of retries to use once the initial connection fails, defaults to 2 if nothing is provided.")
            .short("r")
            .long("retries")
            .takes_value(true)
        )
        .arg(Arg::with_name("extensions")
            .help("extensions that should be tried ")
            .short("x")
            .long("extensions")
            .multiple(true)
            .require_delimiter(false)
            .value_delimiter(",")
        )
        .arg(Arg::with_name("http_codes")
            .help("")
            .short("h")
            .long("http_codes")
            .multiple(true)
            .require_delimiter(false)
            .value_delimiter(",")
        )
        .arg(Arg::with_name("recursive_mode")
            .help("Whether or not to traverse into the specific directories found, defaults to false")
            .short("c")
            .takes_value(false)
            .required(false)
        )
}
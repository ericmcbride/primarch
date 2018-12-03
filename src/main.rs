extern crate clap;
extern crate indicatif;
extern crate reqwest;

use clap::{App, Arg};

mod http;
mod utils;

// Main function that runs the run function.  The run function will return a result or error
fn main() {
    match run() {
        Ok(_) => println!("Report coming soon...."),
        Err(e) => {
            panic!("Error {}", e);
        }
    }
}

// Run function that will collect the arguments, and will validate the url, and then either kick
// off a load test, or return an error to the main function
fn run() -> Result<(), Box<::std::error::Error>> {
    let matches = App::new("Primarch - Load-Driver")
        .version("0.1.0")
        .author("Eric McBride <ericmcbridedeveloper@gmail.com>")
        .about("CLI Load Driver")
        .arg(
            Arg::with_name("URL")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("url to load-test"),
        ).arg(
            Arg::with_name("RPS")
                .required(true)
                .takes_value(true)
                .index(2)
                .help("requests per second"),
        ).arg(
            Arg::with_name("HTTP-VERB")
                .required(true)
                .takes_value(true)
                .index(3)
                .help("Request type"),
        ).get_matches();

    // Check url for base http and strip any white space
    let url = utils::parse_url(matches.value_of("URL").unwrap())?;
    let rps = utils::parse_rps(matches.value_of("RPS").unwrap())?;
    let http_verb = matches.value_of("HTTP-VERB").unwrap();
    let client = reqwest::Client::new();

    // #TODO: Add a type argument to allow extendability of load drive types
    let options = http::HttpOptions {
        url: url,
        rps: rps,
        http_verb: http_verb.to_string(),
        client: client,
    };

    match options.url.scheme() {
        "http" | "https" => http::load_drive(options),
        _ => utils::generate_err(format!(
            "Unsupported HTTP Protocol {:?}",
            options.url.scheme()
        )),
    }
}

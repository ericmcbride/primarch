#[macro_use]
extern crate clap;

extern crate indicatif;
extern crate reqwest;

use clap::{clap_app, App, Arg, ArgMatches};

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
    let args = clap_app!(primarch =>
        (version: "1.0")
        (author: "Eric McBride <ericmcbridedeveloper@gmail.com>")
        (about: "Load Driver written in Rust")
        (@arg URL: -u --url +required +takes_value "URL to load test")
        (@arg RPS: -r --requests_per_second +required +takes_value "Requests Per Second")
        (@arg HTTP_VERB: --http_verb +required +takes_value "HTTP Verb")
        (@arg BODY: -b --body +takes_value "Request body file")
        (@arg DURATION: -d --duration +takes_value "Duration of Test in seconds (0 is forever)")
        (@arg HEADER: ... --header +takes_value "Request Headers (multiple can be set")
    ).get_matches();

    let options = set_args(&args)?;

    match options.url.scheme() {
        "http" | "https" => http::load_drive(options),
        _ => utils::generate_err(format!(
            "Unsupported HTTP Protocol {:?}",
            options.url.scheme()
        )),
    }
}

// Sets arguments for HTTP Client.
fn set_args(args: &ArgMatches) -> Result<http::HttpOptions, Box<::std::error::Error>> {
    let url = utils::parse_url(args.value_of("URL").unwrap())?;
    let rps = utils::parse_u64(args.value_of("RPS").unwrap())?;
    let http_verb = args.value_of("HTTP_VERB").unwrap();
    let string_verb = http_verb.to_owned();

    if let Some(body) = args.value_of("BODY") {
        body;
    } else {
        let body = "".to_string();
    };

    let mut duration = "0";
    if let Some(duration) = args.value_of("DURATION") {
        duration;
    }

    let u64_duration = utils::parse_u64(duration).unwrap();

    //let mut owned_headers = Vec::new()
    let mut headers = Vec::new();
    if let Some(headers) = args.values_of("HEADER") {
        headers;
    }

    let owned_headers = utils::str_to_string(headers);

    let client = reqwest::Client::new();
    Ok(http::HttpOptions {
        url: url,
        rps: rps,
        http_verb: string_verb,
        client: client,
        duration: u64_duration,
        headers: owned_headers,
    })
}

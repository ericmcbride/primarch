extern crate clap;
extern crate indicatif;
extern crate reqwest;

use clap::{Arg, App};
use indicatif::{ProgressBar, ProgressStyle};

use std::process;
use reqwest::{Url, UrlError};
use std::io::{Error, ErrorKind, Write};


// Main function that runs the run function.  The run function will return a result or error
fn main() {
    match run() {
        Ok(_) => println!("We need to drive a load below"),
        Err(e) => {
            panic!("Error {}", e);
            process::exit(1)
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
        .arg(Arg::with_name("URL")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("url to load-test"))
        .get_matches();

    // Check url for base http and strip any white space
    let url = parse_url(matches.value_of("URL").unwrap())?;
    println!("Url is {}", url);

    match url.scheme() {
        "http" | "https" => Ok(()), // load_drive function will be called here
        _ => generate_err(format!("Unsupported HTTP Protocol {}", url.scheme())),
    }
}


// #TODO: Move to other file
// This will be the progress bar for the Load Driver
fn create_gui_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    let bar = match quiet_mode {
        true => ProgressBar::hidden(),
        false => {
            match length {
                Some(len) => ProgressBar::new(len),
                None => ProgressBar::new_spinner(),
            }
        }
    };

    bar.set_message(msg);

    //#TODO MAtch statement with what we want to show during load tests to the cli 
    bar
}


// Helper function to generate the Result enum
pub fn generate_err(err_msg: String) -> Result<(), Box<::std::error::Error>> {
    Err(Box::new(Error::new(ErrorKind::Other, err_msg)))
}

// Make sure the url is a valid url.  Returns a result enum, with either a Url type or a UrlError
// type.  If the error is a relative url without a base, primarch will correct that for the end
// user, and dynamically append http:// to the relative url
pub fn parse_url(url: &str) -> Result<Url, UrlError> {
    match Url::parse(url) {
        Ok(url) => Ok(url),
        Err(error) if error == UrlError::RelativeUrlWithoutBase => {
            let formatted_url = format!("{}{}", "http://", url);
            Url::parse(formatted_url.as_str())
        }
        Err(error) => Err(error),
    }
}

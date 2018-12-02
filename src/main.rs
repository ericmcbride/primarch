extern crate clap;
extern crate indicatif;
extern crate reqwest;

use clap::{Arg, App};
use indicatif::{ProgressBar, ProgressStyle};

use std::process;
use reqwest::{Url, UrlError};
use std::io::{Error, ErrorKind, Write};
use std::sync::mpsc;
use std::thread;



// Main function that runs the run function.  The run function will return a result or error
fn main() {
    match run() {
        Ok(_) => println!("Report coming soon...."),
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
        .arg(Arg::with_name("RPS")
                  .required(true)
                  .takes_value(true)
                  .index(2)
                  .help("requests per second"))
        .get_matches();

    // Check url for base http and strip any white space
    let url = parse_url(matches.value_of("URL").unwrap())?;
    let rps = parse_rps(matches.value_of("RPS").unwrap())?;
    
    match url.scheme() {
        "http" | "https" => load_driver(url, rps),
        _ => generate_err(format!("Unsupported HTTP Protocol {}", url.scheme())),
    }
}

// Convert rps from string to i32. Return result enum
fn parse_rps(rps: &str) -> Result<i32, Box<::std::error::Error>> {
    let rps: i32 = rps.parse().unwrap();
    Ok(rps)
}


// Load to be driven to given url. Spins up threads, and hits the url.  Returns a result enum.  
fn load_driver(url: Url, rps: i32) -> Result<(), Box<::std::error::Error>> {
    let client = reqwest::Client::new();
    let (tx, rx) = mpsc::channel();
    
    // #TODO Add a timer to spin up REQUESTS PER SECOND.  Something like a token bucket implemented
    // right here
    for _ in 0..rps {
        let tx = tx.clone();
        let client = client.clone();
        let url = url.clone();
        thread::spawn(move || {
            let res = client.post(url).send().unwrap();
            tx.send(res)
        });
    } 
    
    let mut response_vector = Vec::new();
    for _ in 0..rps {
        let r = rx.recv().unwrap();
        response_vector.push(r);
    }

    println!("Response vector is {:?}", response_vector);
    Ok(()) 
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

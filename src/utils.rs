use clap::ArgMatches;
use http;
use reqwest::{Url, UrlError};
use std::io::{Error, ErrorKind, Read};

use std::fs::File;

// Convert rps from string to u64. Return result enum
fn parse_u64(value: &str) -> Result<u64, Box<::std::error::Error>> {
    let val: u64 = value.parse()?;
    Ok(val)
}

pub fn str_to_string(input: Vec<&str>) -> Vec<String> {
    let mut string_vec = Vec::new();
    for x in input {
        string_vec.push(x.to_owned());
    }
    string_vec
}

// Helper function to generate the Result enum
pub fn generate_err(err_msg: String) -> Result<(), Box<::std::error::Error>> {
    Err(Box::new(Error::new(ErrorKind::Other, err_msg)))
}

// Make sure the url is a valid url.  Returns a result enum, with either a Url type or a UrlError
// type.  If the error is a relative url without a base, primarch will correct that for the end
// user, and dynamically append http:// to the relative url
fn parse_url(url: &str) -> Result<Url, UrlError> {
    match Url::parse(url) {
        Ok(url) => Ok(url),
        Err(error) if error == UrlError::RelativeUrlWithoutBase => {
            let formatted_url = format!("{}{}", "http://", url);
            Url::parse(formatted_url.as_str())
        }
        Err(error) => Err(error),
    }
}

fn get_check(url: Url) -> Result<(), Box<::std::error::Error>> {
    reqwest::get(url)?;
    Ok(())
}

// Create headers for the requests
fn create_reqwest_headers(
    headers: &Vec<String>,
) -> Result<reqwest::header::HeaderMap, Box<::std::error::Error>> {
    let mut new_headers = reqwest::header::HeaderMap::new();

    for head in headers {
        let mut split_vect: Vec<&str> = head.split(":").collect();
        let header_name = reqwest::header::HeaderName::from_bytes(split_vect[0].as_bytes())?;
        let header_value = reqwest::header::HeaderValue::from_bytes(split_vect[1].as_bytes())?;

        new_headers.insert(header_name, header_value);
    }
    Ok(new_headers)
}

// Sets arguments for HTTP Client.
pub fn set_args(args: &ArgMatches) -> Result<http::HttpOptions, Box<::std::error::Error>> {
    let mut url = parse_url(args.value_of("URL").unwrap())?;
    let _ = get_check(url.clone())?;
    let rps = parse_u64(args.value_of("RPS").unwrap())?;
    let http_verb = args.value_of("HTTP_VERB").unwrap();
    let string_verb = http_verb.to_owned();

    let body = if let Some(body) = args.value_of("BODY") {
        let new_body = open_file(body.to_string())?;
        new_body
    } else {
        "".to_string()
    };

    let duration = if let Some(dur) = args.value_of("DURATION") {
        let d = parse_u64(dur)?;
        d
    } else {
        let d = parse_u64("0")?;
        d
    };

    let headers: reqwest::header::HeaderMap = if let Some(_) = args.values_of("HEADER") {
        let h = args.values_of("HEADER").unwrap().collect();
        let owned_headers = str_to_string(h);
        create_reqwest_headers(&owned_headers)?
    } else {
        reqwest::header::HeaderMap::new()
    };

    Ok(http::HttpOptions {
        url: url,
        rps: rps,
        http_verb: string_verb,
        duration: duration,
        headers: headers,
        body: body,
    })
}

fn open_file(body: String) -> Result<String, Box<::std::error::Error>> {
    let mut file = File::open(body)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

use clap::ArgMatches;
use http;
use reqwest::{Url, UrlError};
use std::io::{Error, ErrorKind};

// Convert rps from string to u64. Return result enum
fn parse_u64(value: &str) -> Result<u64, Box<::std::error::Error>> {
    let value: u64 = value.parse().unwrap();
    Ok(value)
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

// Sets arguments for HTTP Client.
pub fn set_args(args: &ArgMatches) -> Result<http::HttpOptions, Box<::std::error::Error>> {
    let url = parse_url(args.value_of("URL").unwrap())?;
    let rps = parse_u64(args.value_of("RPS").unwrap())?;
    let http_verb = args.value_of("HTTP_VERB").unwrap();
    let string_verb = http_verb.to_owned();

    let body = if let Some(body) = args.value_of("BODY") {
        body
    } else {
        ""
    };

    let duration = if let Some(duration) = args.value_of("DURATION") {
        duration
    } else {
        "0"
    };

    let u64_duration = parse_u64(duration).unwrap();

    let headers: Vec<&str> = if let Some(_headers) = args.values_of("HEADER") {
        args.values_of("HEADER").unwrap().collect()
    } else {
        Vec::new()
    };

    let owned_headers = str_to_string(headers);

    Ok(http::HttpOptions {
        url: url,
        rps: rps,
        http_verb: string_verb,
        duration: u64_duration,
        headers: owned_headers,
        body: body.to_string(),
    })
}

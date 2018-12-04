use reqwest::{Url, UrlError};
use std::io::{Error, ErrorKind};

// Convert rps from string to u64. Return result enum
pub fn parse_u64(value: &str) -> Result<u64, Box<::std::error::Error>> {
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

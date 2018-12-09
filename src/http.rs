use reqwest;
use std::sync::mpsc;
use std::thread;

pub struct HttpOptions {
    pub url: reqwest::Url,
    pub rps: u64,
    pub http_verb: String,
    pub duration: u64,
    pub headers: reqwest::header::HeaderMap,
    pub body: String,
}

// #TODO: See if we can get these 2 functions into an impl for HttpOptions.  Calling the 2
// methods from load_driver seemd to be an issue, even when passing self as a mutable reference.
fn post_request(
    url: reqwest::Url,
    headers: reqwest::header::HeaderMap,
    body: String,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let resp = client.post(url).headers(headers).json(&body).send()?;
    Ok(resp)
}
fn get_request(
    url: reqwest::Url,
    headers: reqwest::header::HeaderMap,
    _: String,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let resp = client.get(url).headers(headers).send()?;
    Ok(resp)
}

pub trait LoadDriver {
    fn load_driver(&self) -> Result<(), Box<::std::error::Error>>;
}

impl LoadDriver for HttpOptions {
    fn load_driver(&self) -> Result<(), Box<::std::error::Error>> {
        let (tx, rx) = mpsc::channel();
        let rps = self.rps.clone();

        let resp = match self.http_verb.as_ref() {
            "POST" => post_request,
            "GET" => get_request,
            _ => get_request, // defaults
        };

        for _ in 0..rps {
            let tx = tx.clone();
            let url = self.url.clone();
            let headers = self.headers.clone();
            let body = self.body.clone();

            thread::spawn(move || {
                tx.send(resp(url, headers, body))
            });
        }

        for _ in 0..rps {
            // #TODO Decide what statistics we want to report
            println!("Received: {:?}", rx.recv()?);
        }

        Ok(())
    }
}

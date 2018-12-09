use reqwest::header::HeaderMap;
use reqwest::{Client, Error, Response, Url};
use std::sync::mpsc;
use std::thread;
use std::time::{Instant};


pub struct HttpOptions {
    pub url: reqwest::Url,
    pub rps: u64,
    pub http_verb: String,
    pub duration: u64,
    pub headers: HeaderMap,
    pub body: String,
}

// #TODO: See if we can get these 2 functions into an impl for HttpOptions.  Calling the 2
// methods from load_driver seemd to be an issue, even when passing self as a mutable reference.
fn post_request(client: Client, url: Url, headers: HeaderMap, body: String) -> Result<Response, Error> {
    let resp = client.post(url).headers(headers).json(&body).send()?;
    Ok(resp)
}
fn get_request(client: Client, url: Url, headers: HeaderMap, _: String) -> Result<Response, Error> {
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
        let client = Client::new();

        let resp = match self.http_verb.as_ref() {
            "POST" => post_request,
            "GET" => get_request,
            _ => get_request, // defaults
        };
        
        println!("Processing requests...");
        let now = Instant::now();
        for _ in 0..rps {
            let tx = tx.clone();
            let url = self.url.clone();
            let headers = self.headers.clone();
            let body = self.body.clone();
            let client = client.clone();
            thread::spawn(move || tx.send(resp(client, url, headers, body)));
        }
        
        println!("Took {} ms to process", now.elapsed().subsec_millis());
        let mut count = 0;
        let mut err_count = 0;

        for _ in 0..rps {
            let resp = rx.recv()?;
            match resp {
                Ok(_) => count += 1,
                Err(_) => err_count += 1, // #TODO establish baselines for whats an error
            }
        }
        println!("Count is {}", count);
        println!("Err count is {}", err_count);
        Ok(())
    }
}

pub struct LoadReport {
    pub success_count: u64,
    pub error_count: u64,
}

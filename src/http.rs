use reqwest;
use std::sync::mpsc;
use std::thread;

pub struct HttpOptions {
    pub url: reqwest::Url,
    pub rps: u64,
    pub http_verb: String,
    pub client: reqwest::Client,
    pub duration: u64,
    pub headers: Vec<String>,
    pub body: String,
}

// #TODO Make an Impl and A trait for the below helper methods
fn post_request(
    client: reqwest::Client,
    url: reqwest::Url,
    body: String,
    headers: Vec<String>,
) -> reqwest::Response {
    client.post(url).send().unwrap()
}

fn get_request(
    client: reqwest::Client,
    url: reqwest::Url,
    body: String,
    headers: Vec<String>,
) -> reqwest::Response {
    client.get(url).send().unwrap()
}

pub fn load_drive(http: HttpOptions) -> Result<(), Box<::std::error::Error>> {
    let client = reqwest::Client::new();
    let (tx, rx) = mpsc::channel();

    // #TODO Add a timer to spin up REQUESTS PER SECOND.  Something like a token bucket implemented
    // right here

    // default to get if not post
    let http_fn = get_request;
    if http.http_verb == "POST" {
        let http_fn = post_request;
    }
    let rps = http.rps.clone();
    for _ in 0..http.rps {
        let tx = tx.clone();
        let http_fn = http_fn.clone();
        let body = http.body.clone(); //#TODO Prep for reqwest::Body type
        let client = http.client.clone();
        let headers = http.headers.clone(); //#TODO Prep for reqwest::Headers type
        let url = http.url.clone();

        thread::spawn(move || {
            let res = http_fn(client, url, body, headers);
            tx.send(res);
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

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

// #TODO Make an Impl and A trait for the below helper methods
fn post_request(
    client: reqwest::Client,
    url: reqwest::Url,
    headers: reqwest::header::HeaderMap,
    body: String,
) -> reqwest::Response {
    client
        .post(url)
        .headers(headers)
        .json(&body)
        .send()
        .unwrap()
}

fn get_request(
    client: reqwest::Client,
    url: reqwest::Url,
    headers: reqwest::header::HeaderMap,
    _: String,
) -> reqwest::Response {
    client.get(url).headers(headers).send().unwrap()
}

pub fn load_drive(http: HttpOptions) -> Result<(), Box<::std::error::Error>> {
    let (tx, rx) = mpsc::channel();
    let client = reqwest::Client::new();

    //#TODO Fix this logic right now its just GET
    let http_fn = get_request;
    if http.http_verb == "POST" {
        let http_fn = post_request;
    }

    for _ in 0..http.rps {
        let tx = tx.clone();
        let client = client.clone();
        let url = http.url.clone();
        let headers = http.headers.clone();
        let body = http.body.clone();
        thread::spawn(move || {
            let res = http_fn(client, url, headers, body);
            tx.send(res);
        });
    }

    let mut response_vector = Vec::new();
    for _ in 0..http.rps {
        let r = rx.recv().unwrap();
        response_vector.push(r);
    }

    println!("Response vector is {:?}", response_vector);
    Ok(())
}

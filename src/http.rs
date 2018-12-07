use reqwest;
use std::sync::mpsc;
use std::thread;

pub struct HttpOptions {
    pub url: reqwest::Url,
    pub rps: u64,
    pub http_verb: String,
    pub duration: u64,
    pub headers: Vec<String>,
    pub body: String,
}

fn create_reqwest_headers(headers: &Vec<String>) -> reqwest::header::HeaderMap {
    let mut new_headers = reqwest::header::HeaderMap::new();

    for head in headers {
        let mut split_vect: Vec<&str> = head.split(":").collect();
        let header_name =
            reqwest::header::HeaderName::from_bytes(split_vect[0].as_bytes()).unwrap();
        let header_value =
            reqwest::header::HeaderValue::from_bytes(split_vect[1].as_bytes()).unwrap();

        new_headers.insert(header_name, header_value);
    }
    new_headers
}

// #TODO Make an Impl and A trait for the below helper methods
fn post_request(
    client: reqwest::Client,
    url: reqwest::Url,
    headers: reqwest::header::HeaderMap,
) -> reqwest::Response {
    client.post(url).headers(headers).send().unwrap()
}

fn get_request(
    client: reqwest::Client,
    url: reqwest::Url,
    headers: reqwest::header::HeaderMap,
) -> reqwest::Response {
    client.get(url).headers(headers).send().unwrap()
}

pub fn load_drive(http: HttpOptions) -> Result<(), Box<::std::error::Error>> {
    let (tx, rx) = mpsc::channel();
    let client = reqwest::Client::new();

    let headers = create_reqwest_headers(&http.headers);

    //#TODO Fix this logic
    let http_fn = get_request;
    if http.http_verb == "POST" {
        let http_fn = post_request;
    }

    for _ in 0..http.rps {
        let tx = tx.clone();
        let client = client.clone();
        let url = http.url.clone();
        let headers = headers.clone();
        thread::spawn(move || {
            let res = http_fn(client, url, headers);
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

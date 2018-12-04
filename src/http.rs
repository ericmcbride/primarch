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
}

// #TODO Make an Impl and A trait for the below helper methods
fn post_request(client: reqwest::Client, url: reqwest::Url) -> reqwest::Response {
    client.post(url).send().unwrap()
}

fn get_request(client: reqwest::Client, url: reqwest::Url) -> reqwest::Response {
    client.get(url).send().unwrap()
}

pub fn load_drive(http: HttpOptions) -> Result<(), Box<::std::error::Error>> {
    let client = reqwest::Client::new();
    let (tx, rx) = mpsc::channel();

    // #TODO Add a timer to spin up REQUESTS PER SECOND.  Something like a token bucket implemented
    // right here
    for _ in 0..http.rps {
        let tx = tx.clone();
        let client = client.clone();
        let url = http.url.clone();
        //let http_verb = http.http_verb.clone();
        let http_verb = "POST".to_string();
        thread::spawn(move || {
            if http_verb == "POST" {
                let res = post_request(client, url);
                tx.send(res);
            } else {
                let res = get_request(client, url);
                tx.send(res);
            }
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

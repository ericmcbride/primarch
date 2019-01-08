use time::PreciseTime;

use indicatif::ProgressBar;
use reqwest::header::HeaderMap;
use reqwest::{Client, Error, Response, Url};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

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
fn post_request(
    client: Client,
    url: Url,
    headers: HeaderMap,
    body: String,
) -> Result<Response, Error> {
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

        //#TODO Improve performance of this whole section and code quality and then add the
        //infinite version of it for long running processes.  Maybe implement a limiter/token
        //bucket to see if we can make the requests more spread out and less bursty
        println!("Processing requests...");
        let now = Instant::now();
        let dur = Duration::new(self.duration, 0);
        let mut count = 0;
        let mut err_count = 0;
        let pb = ProgressBar::new(self.duration * self.rps);

        while now.elapsed() <= dur {
            let execution_time = Instant::now();
            for i in 0..rps {
                pb.inc(1);
                let tx = tx.clone();
                let url = self.url.clone();
                let headers = self.headers.clone();
                let body = self.body.clone();
                let client = client.clone();

                thread::spawn(move || tx.send(time_request(resp, client, url, headers, body)));

                // If the requests are equal, and the time passed is less then a second we need to throttle
                if i == (rps - 1) && now.elapsed() <= dur {
                    let elapsed_time = Instant::now();
                    let sleep_time =
                        1000 as u32 - elapsed_time.duration_since(execution_time).subsec_millis();
                    let sleep_ms = Duration::from_millis(sleep_time as u64);
                    thread::sleep(sleep_ms);
                }
            }

            for _ in 0..rps {
                let resp = rx.recv()?;
                println!("Resp time is: {:?}ms", resp.num_milliseconds());
            }
        }
        Ok(())
    }
}

pub fn time_request<F>(
    f: F,
    client: Client,
    url: Url,
    header_map: HeaderMap,
    body: String,
) -> time::Duration
where
    F: Fn(Client, Url, HeaderMap, String) -> Result<Response, Error>,
{
    let start = PreciseTime::now();
    let resp = f(client, url, header_map, body);
    let end = PreciseTime::now();
    start.to(end)
}

pub struct LoadReport {
    pub success_count: u64,
    pub error_count: u64,
}

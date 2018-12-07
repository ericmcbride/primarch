#[macro_use]
extern crate clap;

extern crate indicatif;
extern crate reqwest;

mod http;
mod utils;

// Main function that runs the run function.  The run function will return a result or error
fn main() {
    match run() {
        Ok(_) => println!("Report coming soon...."),
        Err(e) => {
            panic!("Error {}", e);
        }
    }
}

// Run function that will collect the arguments, and will validate the url, and then either kick
// off a load test, or return an error to the main function
fn run() -> Result<(), Box<::std::error::Error>> {
    let args = clap_app!(primarch =>
        (version: "1.0")
        (author: "Eric McBride <ericmcbridedeveloper@gmail.com>")
        (about: "Load Driver written in Rust")
        (@arg URL: -u --url +required +takes_value "URL to load test")
        (@arg RPS: -r --requests_per_second +required +takes_value "Requests Per Second")
        (@arg HTTP_VERB: --http_verb +required +takes_value "HTTP Verb")
        (@arg BODY: -b --body +takes_value "Request body file")
        (@arg DURATION: -d --duration +takes_value "Duration of Test in seconds (0 is forever)")
        (@arg HEADER: ... --header +takes_value "Request Headers (multiple can be set")
    ).get_matches();

    let body = if let Some(body) = args.value_of("BODY") {
        body
    } else {
        ""
    };
    println!("body is {:?}", body);
    let options = utils::set_args(&args)?;

    match options.url.scheme() {
        "http" | "https" => http::load_drive(options),
        _ => utils::generate_err(format!(
            "Unsupported HTTP Protocol {:?}",
            options.url.scheme()
        )),
    }
}

# Primarch
[![CircleCI](https://circleci.com/gh/ericmcbride/primarch/tree/master.svg?style=svg)](https://circleci.com/gh/ericmcbride/primarch/tree/master)

Primarch is a load drive tool written in Rust.


## Current Usage
``` bash
cargo run -- --url google.com --requests_per_second 5 --http_verb GET --header Foo:bar --duration=10
```


## Arguments

REQUIRED:
- URL --url: URL of site to be load tested (single value)
- RPS, --requests_per_second: How many requests per second (single value)
- HTTP_VERB, --http_verb: GET or POST supported right now (single value)

OPTIONAL:
- BODY, --body: JSON file to open for payload body (FORM suport coming soon) (single value)
- DURATION, --duration: Length of load test (infinite support coming soon) (single value)
- HEADER, --header: Key Value seperated by a Colon (:).  Example: Auth:1234 (multiple value) 


## The Future
1.) Make a lib to be used
2.) Read config files
3.) Make more generic / easier for extendability
4.) Native GRPC Support

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

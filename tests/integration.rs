use std::process::Command;

static WITHOUT_ARGS_OUTPUT: &'static str =
    "error: The following required arguments were not provided:
    --http_verb <HTTP_VERB>
    --requests_per_second <RPS>
    --url <URL>

USAGE:
    primarch [OPTIONS] --http_verb <HTTP_VERB> --requests_per_second <RPS> --url <URL>

For more information try --help
";

static WITHOUT_URL_ARGS_OUTPUT: &'static str =
    "error: The following required arguments were not provided:
    --url <URL>

";

static WITHOUT_RPS_ARGS_OUTPUT: &'static str =
    "error: The following required arguments were not provided:
    --requests_per_second <RPS>
";

static WITHOUT_VERB_ARGS_OUTPUT: &'static str =
    "error: The following required arguments were not provided:
    --http_verb <HTTP_VERB>
";

#[cfg(test)]
mod integration {
    use Command;
    use WITHOUT_ARGS_OUTPUT;
    use WITHOUT_RPS_ARGS_OUTPUT;
    use WITHOUT_URL_ARGS_OUTPUT;
    use WITHOUT_VERB_ARGS_OUTPUT;

    #[test]
    fn calling_primarch_without_args() {
        let output = Command::new("./target/debug/primarch")
            .output()
            .expect("failed to execute process");

        assert_eq!(String::from_utf8_lossy(&output.stderr), WITHOUT_ARGS_OUTPUT);
    }

    #[test]
    fn calling_primarch_without_url() {
        let output = Command::new("./target/debug/primarch")
            .args(&["--requests_per_second", "5"])
            .args(&["--http_verb", "POST"])
            .output()
            .expect("failed to execute process");
        assert!(String::from_utf8_lossy(&output.stderr).contains(WITHOUT_URL_ARGS_OUTPUT));
    }

    #[test]
    fn calling_primarch_without_rps() {
        let output = Command::new("./target/debug/primarch")
            .args(&["--url", "google.com"])
            .args(&["--http_verb", "POST"])
            .output()
            .expect("failed to execute process");
        assert!(String::from_utf8_lossy(&output.stderr).contains(WITHOUT_RPS_ARGS_OUTPUT));
    }

    #[test]
    fn calling_primarch_without_verb() {
        let output = Command::new("./target/debug/primarch")
            .args(&["--url", "google.com"])
            .args(&["--requests_per_second", "5"])
            .output()
            .expect("failed to execute process");
        assert!(String::from_utf8_lossy(&output.stderr).contains(WITHOUT_VERB_ARGS_OUTPUT));
    }

}

pub mod utils {
    use std::io::{Error, ErrorKind};

    // Helper function to generate the Result enum
    pub fn generate_err(err_msg: String) -> Result<(), Box<::std::error::Error>> {
        Err(Box::new(Error::new(ErrorKind::Other, err_msg)))
    }
}

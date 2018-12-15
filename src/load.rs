pub mod load {
    pub trait LoadDriver {
        fn load_driver(&self) -> Result<(), Box<::std::error::Error>>;
    }
}

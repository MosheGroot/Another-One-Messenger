#[derive(Debug)]
pub struct Config {
    pub api_url: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            api_url: "http://localhost:8080".to_string(),
        }
    }
}

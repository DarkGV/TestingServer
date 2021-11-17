use serde::Deserialize;
use toml;
use std::fs;

// Very simple configuration for now
#[derive(Deserialize)]
pub struct Configuration {
    environment: String, // dev, stg, prod, local
    server_configuration: ServerConfiguration,
}

#[derive(Deserialize)]
pub struct ServerConfiguration {
    host: Option<String>, // any, local
    port: u32,
}

pub fn load_configuration(filename: String) -> Option<Configuration> {
    let configuration = get_file_content(filename).unwrap(); // Get the value or exit panicking
    let parsed_configuration: Configuration = toml::from_str(&configuration).unwrap();
    Some(parsed_configuration)
}

// Internal Function
fn get_file_content(filename: String) -> Option<String> {
    match fs::read_to_string(filename) {
        Ok(content) =>
            Some(content),
        Err(_e) =>
            None,
    }
}

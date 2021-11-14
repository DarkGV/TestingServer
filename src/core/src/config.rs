use serde::Deserialize;
use toml;
use std::fs:File;

pub mod config {
    // Very simple configuration for now
    #[derive(Deserialize)]
    struct Configuration {
        environment: String, // dev, stg, prod, local
        server_configuration: ServerConfiguration,
    }

    #[derive(Deserialize)]
    struct ServerConfiguration {
        host: Option<String>,
        port: u32,
    }

    pub fn load_configuration(filename: String) -> Configuration {
        let configuration_file = File::open(filename)?; // Get the value or exit panicking
        let mut configuration = String::new();
        configuration_file.read_to_string(&mut configuration);
        toml::from_str(configuration).unwrap() as Configuration
    }
}

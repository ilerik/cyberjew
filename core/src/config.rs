use std::fs::File;
use std::io::prelude::*;

use errors::*;
use toml;

#[derive(Deserialize)]
pub struct Configuration {
    neo4j_addr: String,
    mongodb_addr: String,
}

impl Configuration {
    pub fn from_file(mut f: File) -> Result<Configuration> {
        let mut contents = String::new();
        f.read_to_string(&mut contents);
        let config: Configuration = toml::from_str(&contents)?;
        Ok(config)
    }
}
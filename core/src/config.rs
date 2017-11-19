use std::fs::File;
use std::io::prelude::*;

use errors::*;
use toml;

#[derive(Debug, Deserialize)]
/// Public part of configuration
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

#[derive(Debug, Deserialize)]
// Secret part of configuration
pub struct Secrets {
    master_key: String,
    github_personal_token: String,
}

impl Secrets {
    pub fn from_file(mut f: File) -> Result<Secrets> {
        let mut contents = String::new();
        f.read_to_string(&mut contents);
        let secrets: Secrets = toml::from_str(&contents)?;
        Ok(secrets)
    }
}
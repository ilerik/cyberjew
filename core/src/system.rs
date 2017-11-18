use std::fs::File;

use clients::neo4j::Neo4jDBClient;

use errors::*;
use config::Configuration;

/// System startup phases
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LaunchStage {
    ConfigLoad,
    ConfigParse,
    ConfigResolve,
}

/// Main object that encapsulates cyberjew identity and logic
pub struct System {
}

// Startup sequence and bootstraping
impl System {
    // Constructor
    pub fn new() -> System {        
        System {}
    }

    /// Read the service config from the file specified
    fn load_config(&self, rel_path: &str) -> Result<Configuration> {
        File::open(rel_path)
            .map(|f| {
                let config = Configuration::from_file(f).unwrap();
                config
            })
            .chain_err(|| ErrorKind::ConfigLoad(rel_path.to_string()))
    }

    /// Launch the service
    pub fn launch(&self, _addr: &str, rel_path: &str) -> Result<()> {

        // Load configuration
        let config = self.load_config(rel_path).map_err(|e| match e {
                                        e @ Error(ErrorKind::ConfigLoad(_), _) => {
                                            e.chain_err(|| LaunchStage::ConfigLoad)
                                        }
                                        e => e.chain_err(|| "Unknown failure during loading configuration"),
                                    })?;

        // Start event loop
        

        Ok(())
    }
}
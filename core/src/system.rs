use std::fs::File;
use tokio_core::reactor::Handle;
use futures::{Future, Stream};

use clients::neo4j::Neo4jDBClient;
use server::Server;
use hyper::server::Http;

use errors::*;
use config::{Configuration, Secrets};

/// System startup phases
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LaunchStage {
    ConfigLoad,
    ConfigParse,
    ConfigResolve,
    SecretsLoad,
    Bootstrap,
    Verification,
    Start,
}

/// Main object that encapsulates cyberjew identity and logic
pub struct System {
    pub config: Configuration,
    secrets: Secrets,
}

// Utility functions
impl System {
    /// Read the service config from the file specified
    pub fn load_config(rel_path: &str) -> Result<Configuration> {
        File::open(rel_path)
            .map(|f| {
                let config = Configuration::from_file(f).unwrap();
                config
            })
            .chain_err(|| ErrorKind::ConfigLoad(rel_path.to_string()))
    }

    /// Fetch secret information
    pub fn fetch_secrets(rel_path: &str) -> Result<Secrets> {
        File::open(rel_path)
            .map(|f| {
                let secrets = Secrets::from_file(f).unwrap();
                secrets
            })
            .chain_err(|| ErrorKind::ConfigLoad(rel_path.to_string()))
    }
}

// Startup sequence and bootstraping
impl System {
    /// Discover other public blockchains and bootstrap startup sequence
    fn bootstrap(&self) -> Result<()> {
        Ok(())
    }

    /// REST API interface and process instantiation
    pub fn server(&self, _handle: &Handle, _addr: &str) -> Result<()> {
        let addr = _addr.parse().unwrap();        
        let server = Http::new().serve_addr_handle(&addr, &_handle, || Ok(Server{})).unwrap();
        println!("Listening on http://{}", server.incoming_ref().local_addr());

        let handle = _handle.clone();
        _handle.spawn(server.for_each(move |conn| {
            handle.spawn(conn.map(|_| ()).map_err(|err| println!("srv1 error: {:?}", err)));
            Ok(())
        }).map_err(|_| ()));
        
        Ok(())
    }

    /// Launch the service and validate configuration
    pub fn new(config_path: &str, secrets_path: &str) -> Result<System> {
        // Load configuration
        let config = System::load_config(config_path).map_err(|e| match e {
                                        e @ Error(ErrorKind::ConfigLoad(_), _) => {
                                            e.chain_err(|| LaunchStage::ConfigLoad)
                                        }
                                        e => e.chain_err(|| "Unknown failure during loading configuration"),
                                    })?;
        
        // Fetch secrets private data
        let secrets = System::fetch_secrets(secrets_path).map_err(|e| match e {
                                        e @ Error(ErrorKind::ConfigLoad(_), _) => {
                                            e.chain_err(|| LaunchStage::SecretsLoad)
                                        }
                                        e => e.chain_err(|| "Unknown failure during fetchin secrets configuration"),
                                    })?;

        // Construct system object
        let system = System {
            config: config,
            secrets: secrets,
        };

        // Discover public blockchains, database instances, verify infrastructure and run consistency checks
        system.bootstrap()?;

        // Return created instance
        Ok(system)
    }
}
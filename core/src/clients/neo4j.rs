use std::collections::BTreeMap;
use rusted_cypher::GraphClient;
use rusted_cypher::cypher::Statement;

use errors::*;

// Neo4j database instance controller
pub struct Neo4jDBClient {
    pub addr: Option<String>, // address of neo4j instance API endpoint
    pub user: Option<String>, // optional credentials username
    pub pass: Option<String>, // optional credentials password
    graph: Option<GraphClient>, // graph database connection client
}

// Implement constructor that connects to endpoint
impl Neo4jDBClient {
    pub fn new() -> Neo4jDBClient {        
        // Return instance of client
        Neo4jDBClient{
            addr: None,
            user: None, // no user
            pass: None, // no pass
            graph: None,
        }
    }

    /// Establish connection with database enpoint
    pub fn connect(&mut self, _addr: &str) -> Result<()> {
        // Connect to the database
        self.graph = Some(GraphClient::connect(_addr).unwrap());
        self.addr = Some(_addr.to_string());
        Ok(())
    }
}
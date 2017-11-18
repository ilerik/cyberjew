// allow modern futures impl Trait mechanics
#![feature(conservative_impl_trait)]
// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use] extern crate error_chain;
extern crate log;
extern crate tokio_core;
extern crate futures;
extern crate hyper;

#[macro_use] extern crate rusted_cypher;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate toml;

extern crate github_rs;

//extern crate juice;
//extern crate coaster as co;

pub mod errors;
pub mod config;
pub mod clients;
pub mod system;
pub mod crypto;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
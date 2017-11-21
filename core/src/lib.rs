// allow modern futures impl Trait mechanics
#![feature(conservative_impl_trait)]
// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use] extern crate error_chain;
extern crate log;
extern crate tokio_core;
extern crate futures;
extern crate hyper;
extern crate toml;
//extern crate telegram_bot;
#[macro_use] extern crate rusted_cypher;
#[macro_use] extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

extern crate github_rs;

//extern crate juice;
//extern crate coaster as co;

pub mod config;
pub mod errors;
pub mod clients;
pub mod server;
pub mod system;
pub mod crypto;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
#![feature(use_extern_macros)]

extern crate cyberjew;

extern crate pretty_env_logger;
extern crate futures;
extern crate tokio_core;

use std::rc::Rc;
use std::env;

use futures::future;
use futures::Stream;
use tokio_core::reactor::Core;

use cyberjew::system::System as Cyberjew;
use cyberjew::errors::*;

//Entrypoint function that facilitates error handling
fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}

// CyberJew bootstrap and event loop
// Most functions will return the `Result` type, imported from the
// `errors` module. It is a typedef of the standard `Result` type
// for which the error type is always our own `Error`.
fn run() -> Result<()> {
    // Some preparations and configuration steps
    pretty_env_logger::init()?;

    // Create the event loop that will drive this server
    let mut core = Core::new()?;
    let handle = core.handle();

    // Spin up the server on the event loop    
    core.run(future::done({
        // instantiate cyberjew
        let cyberjew = Cyberjew::new("./example.config.toml", "./secrets.toml")?;

        // run REST API server
        cyberjew.server("127.0.0.1")
    }))?;

    Ok(())
}
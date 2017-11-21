use futures;
use futures::{Future, Stream};
use futures::future::FutureResult;

use hyper;
use hyper::{Get, Post, StatusCode, Error};
use tokio_core::reactor::Core;
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};

pub struct Server {
}

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        futures::future::ok(match (req.method(), req.path()) {
            (&Get, "/") => {
                let greeting = "CyberJew greets you!";
                Response::new()
                    .with_header(ContentLength(greeting.len() as u64))
                    .with_body(greeting)
            },
            // // Authentication endpoints
            // (&Get, "/auth") => {

            // },
            // // Get authentication challenge (availible to anyone)
            // (&Post, "/auth/request") => {
                
            // },
            // // Post authentification challenge response
            // (&Post, "/auth/response") => {

            // },
            _ => {
                Response::new()
                    .with_status(StatusCode::NotFound)
            }
        })
    }

}
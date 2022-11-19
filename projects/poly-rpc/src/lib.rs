#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

mod defines;
mod errors;

pub use errors::{Error, Result};

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use std::{convert::Infallible, net::SocketAddr};

async fn handle(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World!".into()))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

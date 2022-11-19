#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

mod defines;

use hyper::{Body, Request, Response, StatusCode};

async fn hello_world(_: Request<Body>) -> Result<Response<Body>, StatusCode> {
    Ok(Response::new("Hello, World!".into()))
}

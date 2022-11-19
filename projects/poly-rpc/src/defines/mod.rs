use std::net::SocketAddr;

use hyper::{Body, HeaderMap, Method, Request, Response, StatusCode};
use hyper::header::{CONTENT_TYPE, HeaderValue};
use crate::errors::PolyResult;

mod der;

#[derive(Debug)]
pub struct HelloRequest {
    id: usize,
}

impl Default for HelloRequest {
    fn default() -> Self {
        Self { id: 0 }
    }
}



impl HelloRequest {
    fn with_path(mut self, id: &str) -> Self {
        todo!()
    }
    fn with_query(mut self, query: &str) {
        todo!()
    }
}

pub struct HelloResponse {}

/// service world {
///     hello(name: String) -> HelloResponse
/// }
pub trait World {
    // get{}
    async fn get_hello(&mut self, input: HelloRequest) -> HelloResponse;
}

pub struct HelloContext {
    context: String,
}

impl World for HelloContext {
    async fn get_hello(&mut self, name: HelloRequest) -> HelloResponse {
        todo!()
    }
}

pub struct RouteDefine {}

pub struct GameServer {
    pub(crate) socket: SocketAddr,
    pub(crate) base: String,
    pub(crate) hello: HelloContext,
}

impl GameServer {
    pub fn as_server(&self) {}

    fn resolve_route(&mut self, req: Request<Body>) -> PolyResult {
        let mut response = Response::new(Body::empty());
        let url = req.uri();
        let query = url.query();
        resolve_body(req.body(), req.headers());
        match req.method() {
            &Method::GET => match vec!["user", "128", "icon", "256"].as_slice() {
                &["user", "not"] => {
                    HelloRequest::default()
                        .with_path()
                        .with_query()

                    self.hello.get_hello()
                }
                _ => {}
            },
            _ => {
                *response.status_mut() = StatusCode::NOT_FOUND;
            }
        }
        Ok(())
    }

}

fn resolve_body(body: &Body, headers: &HeaderMap) {
    match headers.get(CONTENT_TYPE) {
        Some(_) => {},
        None => {}
    }
}
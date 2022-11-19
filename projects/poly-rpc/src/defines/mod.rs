use std::collections::BTreeMap;
use std::net::SocketAddr;

use hyper::{Body, HeaderMap, Method, Request, Response, StatusCode};
use hyper::header::{CONTENT_TYPE, HeaderValue};
use serde::de::Visitor;
use poly_rt::PolyResult;
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
    fn with_query(mut self, query: &BTreeMap<String, String>) -> PolyResult<Self> {
        match query.get("id") {

            Some(_) => {},
            None => {

            }
        }
        self.id = query.get()
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
        let mut queries = BTreeMap::default();

        resolve_body(req.body(), req.headers());
        match req.method() {
            &Method::GET => match vec!["user", "128", "icon", "256"].as_slice() {
                &["user", user_id, "not", name] => {
                    queries.insert("user_id", user_id);

                    HelloRequest::default()
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


pub struct QueryBuilder {

}


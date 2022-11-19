use std::net::SocketAddr;

use serde::{de::Visitor, Deserialize};

use poly_rt::{Body, Method, PolyResult, QueryBuilder, Request, Response, StatusCode};

mod der;

#[derive(Debug, Deserialize)]
pub struct HelloRequest {
    id: usize,
}

impl Default for HelloRequest {
    fn default() -> Self {
        Self { id: 0 }
    }
}

impl HelloRequest {}

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

    async fn resolve_route(&mut self, req: Request<Body>) -> PolyResult {
        let mut response = Response::new(Body::empty());
        let path = QueryBuilder::uri_path(&req, "/aa")?;
        match req.method() {
            &Method::GET => match path.as_slice() {
                ["user", user_id, "not", name] => {
                    let mut queries = QueryBuilder::from_uri(&req, &[]);
                    queries.insert_path("user_id", user_id);
                    queries.insert_path("name", name);
                    self.hello.get_hello(HelloRequest::deserialize(queries.inner)?).await;
                }
                ["user", user_id, "not", name @ ..] => {
                    let mut queries = QueryBuilder::from_uri(&req, &[]);
                    queries.insert_path("user_id", *user_id);
                    queries.insert_path("name", name);
                    self.hello.get_hello(queries.cast_to::<HelloRequest>()?).await;
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

use std::net::SocketAddr;

use serde::{de::Visitor, Deserialize, Serialize};

use poly_rt::{Body, Method, PolyResult, QueryBuilder, Request, Response, Server, StatusCode};

use crate::WorldContext;

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

#[derive(Debug, Serialize)]
pub struct HelloResponse {}

impl Default for HelloResponse {
    fn default() -> Self {
        Self {}
    }
}

pub trait WorldService {
    // get{}
    async fn get_hello(&mut self, input: HelloRequest) -> HelloResponse;
}

pub struct RouteDefine {}

pub struct GameServer {
    pub(crate) socket: SocketAddr,
    pub(crate) base: String,
    pub(crate) world: WorldContext,
}

impl GameServer {
    pub fn as_server(&self) {
        // todo!()
        Server::bind(&self.socket).serve()
    }

    async fn resolve_route(&mut self, request: Request<Body>) -> PolyResult<Response<Body>> {
        let mut response = Response::new(Body::empty());
        let path = QueryBuilder::uri_path(&request, "/proxy")?;
        match request.method() {
            &Method::GET => match path.as_slice() {
                ["user", user_id, "not", name @ ..] => {
                    let name = QueryBuilder::from_headers(&request, &[])
                        .with_headers(&request.headers(), &["Req"])
                        .with_path("user_id", user_id)
                        .with_path("name", name)
                        .cast_to::<HelloRequest>();
                    self.world.get_hello(name).await?;
                }
                _ => {}
            },
            _ => {
                *response.status_mut() = StatusCode::NOT_FOUND;
            }
        }
        Ok(response)
    }
}

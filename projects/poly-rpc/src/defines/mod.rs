pub struct HelloRequest {}

/// service world {
///     hello(name: String) -> HelloResponse
/// }
pub trait World {
    async fn hello(&mut self, input: HelloRequest) -> HelloResponse;
}

pub struct HelloServer {
    context: String,
}

pub struct HelloResponse {}

impl World for HelloServer {
    async fn hello(&mut self, name: String) -> HelloResponse {
        todo!()
    }
}

#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

mod defines;

pub struct WorldContext {
    context: String,
}

impl World for WorldContext {
    async fn get_hello(&mut self, name: HelloRequest) -> HelloResponse {
        todo!()
    }
}

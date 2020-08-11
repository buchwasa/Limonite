use crate::server::Server;
use std::thread;

mod utils;
mod protocol;
mod server;

fn main() {
    let server = thread::Builder::new()
        .name("Bedrock Server".to_string())
        .spawn(|| {
            let mut server = Server::new("0.0.0.0:19132".to_string());
            server.start();
        })
        .expect("Could not start server!");
    server.join();
}

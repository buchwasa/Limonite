use crate::server::Server;
use std::thread;

mod utils;
mod protocol;
mod server;

fn main() {
    let server_thread = thread::Builder::new()
        .name("Bedrock Server".to_string())
        .spawn(|| {
            Server::new("0.0.0.0:19132".to_string()).start();
        })
        .expect("Could not start server!");
    server_thread.join();
}
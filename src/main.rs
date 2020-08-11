use crate::server::Server;
use std::thread;

mod utils;
mod protocol;
mod server;

fn main() {
    let server_thread = thread::Builder::new()
        .name("bedrock server".to_string())
        .spawn(|| {
            println!("Server started!");
            let mut server = Server::new("0.0.0.0:19132".to_string());
            server.start();
        })
        .expect("Could not start server!");
    server_thread.join();
}
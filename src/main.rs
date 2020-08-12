#[macro_use] extern crate log;
extern crate simplelog;

use crate::server::Server;
use simplelog::*;
use std::thread;
use std::fs::File;

mod raknet;
mod server;

fn main() {
    let mut config = ConfigBuilder::new();
    config.set_time_to_local(true);
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Trace, config.build(), TerminalMode::Mixed),
        WriteLogger::new(LevelFilter::Info, config.build(), File::create("server.log").unwrap())
    ]);

    let server_thread = thread::Builder::new()
        .name("Bedrock Server".to_string())
        .spawn(|| {
            info!("Server starting on port 19132");
            let mut server = Server::new("0.0.0.0:19132".to_string());
            server.start();
        })
        .expect("Could not start server");
    server_thread.join().expect("Failed to join threads");
}
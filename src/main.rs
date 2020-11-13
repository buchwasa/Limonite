#[macro_use] extern crate log;
extern crate simplelog;

use crate::server::Server;
use simplelog::*;
use std::thread;

mod server;
mod protocol;
mod utils;

fn main() {
    let mut config = ConfigBuilder::new();
    config.set_time_to_local(true);
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Trace, config.build(), TerminalMode::Mixed)
    ]).expect("Failed to initialize Logger");

    let server_thread = thread::Builder::new()
        .name("RakNet Server".to_string())
        .spawn(|| {
            info!("Starting RakNet server on port 19132"); //TODO: Config
            Server::new("0.0.0.0:19132".to_string()).start();
        })
        .expect("Could not start RakNet server");
    server_thread.join().expect("Failed to join RakNet thread");
}

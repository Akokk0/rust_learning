mod server;
mod router;
mod handler;

use std::env;

fn main() {
    let http_server = server::Server::new("localhost:3000");
    http_server.run();
}

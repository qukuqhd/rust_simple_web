use handler::Handler;
use server::Server;

pub mod handler;
pub mod router;
pub mod server;
fn main() {
    let mut server_app = Server::new("localhost:3000");
    server_app.run();
}

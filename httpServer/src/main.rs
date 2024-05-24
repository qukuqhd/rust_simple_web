pub mod handler;
pub mod router;
pub mod server;
fn main() {
    server::Server::new("localhost:3000").run();
}

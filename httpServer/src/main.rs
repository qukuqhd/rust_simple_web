pub mod server;
pub mod handler;
pub mod router;
fn main() {
    server::Server::new("localhost:3000").run();
}

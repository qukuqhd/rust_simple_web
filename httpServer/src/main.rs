use http::http_response::HttpResponse;
use server::Server;

pub mod handler;
pub mod router;
pub mod server;
fn main() {
    let mut server_app = Server::new("localhost:3000");
    server_app.get("/ss".into(), |_req| HttpResponse::new("200", None, None));
    server_app.run();
}

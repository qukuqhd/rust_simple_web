use std::{io::Read, net::TcpListener};

use http::http_request::HttpRequest;

use crate::router::Router;

pub struct Server<'a> {
    socket_addr: &'a str,
    router:Router
}
impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Self { socket_addr ,router:Router::new()}
    }
    pub fn run(&mut self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Http Server running on {}", self.socket_addr);
        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");
            let mut read_buf = [0; 2000];
                stream.read(&mut read_buf).unwrap();
                let req: HttpRequest = String::from_utf8(read_buf.to_vec()).unwrap().into();
                self.router.route(req, &mut stream);
        }
    }
    pub fn get(&mut self,path:String,handler_func: fn(&HttpRequest) -> http::http_response::HttpResponse){
        self.router.register_route(http::http_request::Method::GET, path, handler_func)
    }
    pub fn post(&mut self,path:String,handler_func: fn(&HttpRequest) -> http::http_response::HttpResponse){
        self.router.register_route(http::http_request::Method::POST, path, handler_func)
    }

}

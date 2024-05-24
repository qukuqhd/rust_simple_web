use std::{io::Read, net::TcpListener};

use http::http_request::HttpRequest;

use crate::router::router;

pub struct Server<'a>{
    socket_addr:&'a str
}
impl<'a> Server<'a>{
    pub fn new(socket_addr:&'a str)->Self{
        Self { socket_addr }
    }
    pub fn run(&self){
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Http Server running on {}",self.socket_addr);
        for stream in connection_listener.incoming(){
            let mut stream = stream.unwrap();
            println!("Connection established");
            let mut read_buf = [0;200];
            stream.read(&mut read_buf).unwrap();
            let req:HttpRequest = String::from_utf8(read_buf.to_vec()).unwrap().into();
            router::route(req,&mut stream);
        }
    }
}
use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("running on port 3000 ...");
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("connect !");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&buffer).unwrap();
    }
}

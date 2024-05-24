use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    stream.write("hello".as_bytes()).unwrap();
    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();
    println!("{:?}", String::from_utf8((&buffer).to_vec()).unwrap());
}

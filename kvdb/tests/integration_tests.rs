use std::io::prelude::*;
use std::net::TcpStream;

#[test]
fn first_test() {
    let mut stream = TcpStream::connect("127.0.0.1:34254").unwrap();

    let write = stream.write(b"").unwrap();
    let result = stream.read(&mut [0; 128]).unwrap();
}

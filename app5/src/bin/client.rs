use std::net::TcpStream;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:7500").unwrap();

    println!("Connected to server");
}
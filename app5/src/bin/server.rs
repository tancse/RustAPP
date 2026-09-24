use std::net::*;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7500").unwrap();

    println!("Server started on port:7500");

    for stream in listener.incoming(){
        let st = stream.unwrap();
        println!("Client is connected");

        //drop(stream);
    }
}
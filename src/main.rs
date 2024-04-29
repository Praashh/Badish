use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

const LOCAL: &str = "127.0.0.1:6001";

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    match stream.read(&mut buffer) {
        Ok(size) => {
            println!("Received:\n {}", String::from_utf8_lossy(&buffer[..size]));
            stream.write("+OK\r\n".as_bytes()).expect("Failed to write to stream");
        },
        Err(e) => println!("Failed to read from stream: {}", e),
    }
}

fn main() {
    let server = TcpListener::bind(LOCAL).expect("Server Listening Failed");
    println!("Server is listening on {}", LOCAL);

    for stream in server.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream);
                });
            },
            Err(e) => {
                println!("Failed to accept connection: {}", e);
            }
        }
    }
}

extern crate caesar;

use caesar::{TlsTcpListener, TlsTcpStream};
use std::thread;

fn open_socket() {
    let key_path = "/home/one/missions/burp.key.pem";
    let cert_path= "/home/one/missions/burp.crt";

    let listener = TlsTcpListener::bind("127.0.0.1:8080", key_path, cert_path).unwrap();

    fn handle_client(stream: TlsTcpStream) {
        println!("Connection established {:?}", stream);
    }

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream);
                });
            }
            Err(e) => { println!("Connection failed, Error {}", e); }
        }
    }
}
fn main() {
    println!("Starting to listen");
    open_socket()
}

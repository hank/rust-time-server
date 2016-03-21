use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Write;

extern crate time;

fn handle_client(mut stream: TcpStream) {
    println!("Talkin' to {}", stream.peer_addr().unwrap());
    let time = time::now();
    match time::strftime("💖  It is now %Y%m%d %H:%M:%S", &time) {
        Ok(time) => {
            // Send client the current time
            let bytes = time.into_bytes();
            match stream.write(&bytes[..]) {
                Ok(_) => { println!("Successfully wrote to stream") },
                Err(e) => panic!("Failed to write to stream: {}", e),
            }
        },
        Err(e) => panic!("Failed to convert time: {}", e),
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2342").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // Connection succeeded
                    handle_client(stream);
                });
            },
            Err(e) => {
                panic!("Connection failed: {}", e);
            }
        }
    }
}

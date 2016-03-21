use std::net::TcpStream;
use std::io::Read;

fn main() {
    match TcpStream::connect("127.0.0.1:2342") {
        Ok(mut stream) => {
            let mut data: Vec<u8> = vec![];
            match stream.read_to_end(&mut data) {
                Ok(_) => {
                    // Convert bytes in vector to string
                    match String::from_utf8(data) {
                        Ok(s) => println!("{} ", s),
                        Err(e) => panic!("Failed to convert: {}", e),
                    }
                },
                Err(e) => panic!("Failed to get data: {}", e),
            }
        },
        Err(e) => panic!("Failed to connect: {}", e),
    }
}

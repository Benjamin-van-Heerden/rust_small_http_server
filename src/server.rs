use crate::http::Request;
use std::{io::Read, net::TcpListener};

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self) {
        println!("Listening on {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(req) => {}
                                Err(e) => println!("Failed to parse request: {}", e),
                            };
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    };
                }
                Err(e) => println!("Failed to establish connection: {}", e),
            }
        }
    }
}

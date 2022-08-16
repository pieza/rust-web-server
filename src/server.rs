use log::{info};
use std::thread;
use std::io::Read;
use std::net::TcpListener;

use crate::{http::{Request}, handler::Handler};

pub struct Server {
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { 
            addr
        }
    }

    pub fn run(self, mut handler: Handler) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        info!("Listening on {}", self.addr);
        
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    thread::spawn(move || {
                        let mut buffer = [0; 1024];
                        stream.read(&mut buffer).expect("Failed to read from connection");

                        let response = match Request::try_from(&buffer[..]) {
                            Ok(request) => {
                                dbg!(&request);
                                handler.handle_request(&request)
                            }
    
                            Err(e) => {
                                handler.handle_bad_request(&e)
                            }
                        };
                        dbg!(&response);
                        response.send(&mut stream).expect("Failed to send response")
                    });
                },
                Err(e) => println!("Failed to establish a connection: {}", e)
            }
        }
    }
}
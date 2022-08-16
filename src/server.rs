use std::io::Read;
use std::net::TcpListener;
use crate::http::{Request, Response, StatusCode, ParseError};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { 
            addr
        }
    }

    pub fn run(self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        println!("Listening on {}", self.addr);
        
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
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

                    response.send(&mut stream).expect("Failed to send response")
                },
                Err(e) => println!("Failed to establish a connection: {}", e)
            }
        }
    }
}
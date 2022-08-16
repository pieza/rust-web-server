use std::fs;
use log::error;

use super::PATH_SEPARATOR;
use super::http::{Request, Response, StatusCode, Method, ParseError};

#[derive(Debug, Clone, Copy)]
pub struct Handler {
    public_path: &'static str
}

impl Handler {
    pub fn new(public_path: &'static str) -> Self {
        Self {
            public_path
        }
    }

    fn parse_path(&self, file_path: &str) -> String {
        let parsed_path = file_path.replace("/", &*PATH_SEPARATOR);
        format!("{}{}{}", self.public_path, *PATH_SEPARATOR, parsed_path)
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = self.parse_path(file_path);
        match dunce::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler {
    pub fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                path => match self.read_file(path) {
                    Some(body) => Response::new(StatusCode::Ok, Some(body)),
                    None => Response::new(StatusCode::NotFound, None),
                }
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }

    pub fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        error!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

unsafe impl Send for Handler {}
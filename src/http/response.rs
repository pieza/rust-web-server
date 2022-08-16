use std::io::{Write, Result as IoResult};
use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self {
            status_code,
            headers: Vec::new(),
            body,
        }
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        write!(
            stream, 
            "HTTP/1.1 {} {}\r\n", 
            self.status_code, 
            self.status_code.reason_phrase()
        )?;
        for (key, value) in &self.headers {
            write!(stream, "{}: {}\r\n", key, value)?;
        }
        write!(stream, "\r\n")?;
        if let Some(body) = &self.body {
            write!(stream, "{}", body)?;
        }
        Ok(())
    }
}
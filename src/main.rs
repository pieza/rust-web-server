#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

use std::env;
use log::info;
use server::Server;
use handler::Handler;

mod server;
mod http;
mod handler;

lazy_static! {
    static ref PATH_SEPARATOR: String = match  cfg!(windows) {
        true => "\\".to_string(),
        false => "/".to_string(),
    };
    static ref PUBLIC_PATH: String = env::var("PUBLIC_PATH").unwrap_or(format!("{}{}public", env!("CARGO_MANIFEST_DIR"), *PATH_SEPARATOR)); 
}
fn main() {
    let server = Server::new(String::from("127.0.0.1:8000"));
    info!("Public path: {}", *PUBLIC_PATH);
    server.run(Handler::new(PUBLIC_PATH.as_str()));
}
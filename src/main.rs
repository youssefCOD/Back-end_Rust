use std::io::{Read, Write}; 
use std::net::{TcpListener, TcpStream}; 
mod settings;
mod lib;
use lib::http::HttpRequest;
mod utils;
fn main(){
    println!("Running on {}:{}", settings::HOST, settings::PORT);
    let http_response_test = HttpRequest::new(200, "text/plain", "Hello World".to_string());
    println!("Http Response:{}", String::from_utf8_lossy(&http_response_test.serialize()));


}
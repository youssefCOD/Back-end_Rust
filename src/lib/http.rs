use crate::utils;

pub struct HttpRequest{
    status_code: u16,
    content_type: String,
    body: String, 
}


impl HttpRequest {

    pub fn new(status_code: u16, content_type: &str, body: String) -> HttpRequest {
        HttpRequest { status_code, content_type: String::from(content_type), body }
    }

    pub fn serialize(&self)-> Vec<u8>{
        format!("HTTP/{}",
            utils::constants::HTTP_VERSION).into_bytes()
    }
}
use std::fs;

use crate::{
    http::{Method, Request, Response, StatusCode},
    server::Handler,
};

pub struct WebSiteHandler {
    public_path: String,
}

impl WebSiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    pub fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    print!("Restricted access: {}", file_path);
                    None
                }
            }

            Err(_) => None
        }
    }
}

impl Handler for WebSiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h1> Welcome </h1>".to_string())),
                path => match self.read_file(path) {
                    Some(content) => Response::new(StatusCode::Ok, Some(content)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}

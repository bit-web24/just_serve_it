use super::header::Header;
use super::status::Status;
use std::fs;
use std::io::{Result, Write};
use std::net::TcpStream;

pub struct Response {
    socket: TcpStream,
    pub headers: Header,
    pub status: Status,
}

impl Response {
    pub fn new(socket: TcpStream) -> Self {
        let mut headers = Header::new();
        headers.set("Content-Type", "text/plain");

        Self {
            socket,
            headers,
            status: Status::new(),
        }
    }

    pub fn send(&mut self, msg: &str) -> Result<()> {
        self.headers
            .set("Content-Length", msg.len().to_string().as_str());

        let response = format!(
            "{}\r\n{}\r\n{}",
            self.status.to_string(),
            self.headers.to_string(),
            msg
        );
        self.socket.write(response.as_bytes())?;
        Ok(())
    }

    pub fn send_file(&mut self, path: &str) -> Result<()> {
        let content = fs::read_to_string(path)?;

        // Determine MIME type from file extension
        let mime_type = match path.split('.').last().unwrap().trim().into() {
            Some("html") => "text/html",
            Some("css") => "text/css",
            Some("js") => "application/javascript",
            Some("json") => "application/json",
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            _ => "application/octet-stream",
        };

        self.headers.set("Content-Type", mime_type);
        self.headers
            .set("Content-Length", content.len().to_string().as_str());

        let response = format!(
            "{}\r\n{}\r\n{}",
            self.status.to_string(),
            self.headers.to_string(),
            content
        );

        self.socket.write_all(response.as_bytes())?;
        Ok(())
    }
}

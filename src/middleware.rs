use super::{request::Request, routes::Router};
use std::io::Result;

pub mod body_parser;
pub mod serve;

#[allow(unused)]
pub use body_parser::BodyParser;

#[allow(unused)]
pub use serve::ServeStatic;

pub trait Middleware: Send + Sync {
    fn handle(&self, routes: &mut Router, req: &mut Request) -> Result<()>;
}

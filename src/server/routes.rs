use super::method::Method;
use super::{request::Request, response::Response};
use std::collections::HashMap;
use std::io::Result;

#[derive(Clone)]
pub struct Route {
    pub path: String,
    pub method: Method,
    pub callback: Option<fn(Request, Response) -> Result<()>>,
}

impl Route {
    pub fn new(path: &str, method: Method, callback: fn(Request, Response) -> Result<()>) -> Self {
        Route {
            path: String::from(path),
            method,
            callback: Some(callback),
        }
    }
}

#[derive(Clone)]

pub struct Router {
    pub get: HashMap<String, Route>,
    pub post: HashMap<String, Route>,
    pub put: HashMap<String, Route>,
    pub patch: HashMap<String, Route>,
    pub delete: HashMap<String, Route>,
    pub not_found: Option<fn(Request, Response) -> Result<()>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            get: HashMap::new(),
            post: HashMap::new(),
            put: HashMap::new(),
            patch: HashMap::new(),
            delete: HashMap::new(),
            not_found: None,
        }
    }

    pub fn add(&mut self, route: Route) {
        match route.method {
            Method::Get => {
                self.get.entry(route.path.clone()).or_insert(route);
            }
            Method::Post => {
                self.post.entry(route.path.clone()).or_insert(route);
            }
            Method::Put => {
                self.put.entry(route.path.clone()).or_insert(route);
            }
            Method::Patch => {
                self.patch.entry(route.path.clone()).or_insert(route);
            }
            Method::Delete => {
                self.delete.entry(route.path.clone()).or_insert(route);
            }
        }
    }

    pub fn get(&mut self, method: &Method, path: &str) -> Option<&Route> {
        match method {
            Method::Get => self.get.get(path),
            Method::Post => self.post.get(path),
            Method::Put => self.put.get(path),
            Method::Patch => self.patch.get(path),
            Method::Delete => self.delete.get(path),
            _ => None,
        }
    }

    pub fn not_found(&mut self, callback: fn(Request, Response) -> Result<()>) {
        self.not_found = Some(callback);
    }
}

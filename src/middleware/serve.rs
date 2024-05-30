use crate::method;
use crate::request::Request;
use crate::response::Response;
use crate::routes::{Route, Router};

use super::Middleware;
use std::io::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct ServeStatic {
    root: String,
}

impl ServeStatic {
    pub fn dir(root: &str) -> Self {
        Self {
            root: Self::sanitize_path(root).to_str().unwrap().to_string(),
        }
    }

    fn sanitize_path(path: &str) -> PathBuf {
        let mut sanitized_path = PathBuf::new();
        for component in path.split('/') {
            if component == ".." || component == "." {
                // Ignore '..' and '.' components
                continue;
            }
            sanitized_path.push(component);
        }
        sanitized_path
    }
}

impl Middleware for ServeStatic {
    fn handle(&self, routes: &mut Router, req: &mut Request) -> Result<()> {
        if let Some(method::Method::Get) = req.method {
            if routes
                .get(
                    &req.method.as_ref().unwrap(),
                    req.path.as_ref().unwrap().as_str(),
                )
                .is_none()
            {
                let sanitized_path = Self::sanitize_path(req.path.as_ref().unwrap().as_str());
                let file_path = Path::new(&self.root).join(&sanitized_path);

                if let Err(_) = fs::metadata(&file_path) {
                    return Ok(());
                }

                req.path = file_path.to_str().unwrap().to_string().into();

                let route = Route::new(
                    file_path.to_str().unwrap(),
                    req.method.clone().unwrap(),
                    server_static_file,
                );

                routes.add(route);
            }
        }

        Ok(())
    }
}

fn server_static_file(request: Request, mut response: Response) -> Result<()> {
    response.send_file(&request.path.unwrap())?;
    Ok(())
}

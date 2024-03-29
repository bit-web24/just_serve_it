pub mod body;
pub mod header;
pub mod method;
pub mod path;
pub mod request;
pub mod response;
pub mod routes;
pub mod status;
pub mod threadpool;

use method::Method;
use request::Request;
use response::Response;
use routes::{Route, Router};
use std::io::{Read, Result};
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

pub struct Server {
    name: String,
    routes: Arc<Mutex<Router>>,
}

impl Server {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            routes: Arc::new(Mutex::new(Router::new())),
        }
    }

    pub fn listen(&mut self, ip: &str, port: u16, _callback: fn() -> Result<()>) -> Result<()> {
        println!(
            "Server \"{}\" Listening on http://{}:{}",
            self.name, ip, port
        );
        let thread_pool = ThreadPool::new(4);
        let listener = std::net::TcpListener::bind(format!("{}:{}", ip, port))?;

        loop {
            let (mut socket, _) = listener.accept()?;
            let mut buf = [0; 1024];
            let n = socket.read(&mut buf).unwrap();
            let mut request_str = String::from_utf8_lossy(&buf[..n]);
            let req = Request::parse(&mut request_str);
            let mut res = Response::new(socket);

            if req.method.is_none() {
                res.status.status_code = 405;
                res.send(format!("Method {:?} Not Allowed!", req.method).as_str())
                    .unwrap();
                continue;
            }

            let req_path = req.path.as_ref().unwrap();
            let mut routes = self.routes.lock().unwrap();

            let route = routes.get(&req.method.as_ref().unwrap(), req_path);

            fn not_found(req: Request, mut res: Response) -> Result<()> {
                res.status.status_code = 404;
                res.send(format!("Page {} Not Found!", req.path.unwrap()).as_str())
                    .unwrap();
                Ok(())
            }

            let callback: fn(Request, Response) -> Result<()>;

            if let Some(route) = route {
                callback = route.callback.unwrap();
            } else {
                let default_callback = routes.not_found;

                if default_callback.is_some() {
                    callback = default_callback.unwrap();
                } else {
                    callback = not_found;
                }
            }

            thread_pool.execute(move || {
                callback(req, res).unwrap();
            });
        }
    }

    pub fn get(
        &mut self,
        path: &str,
        _callback: fn(Request, Response) -> Result<()>,
    ) -> Result<()> {
        let mut routes = self.routes.lock().unwrap();
        routes.add(Route::new(path, Method::Get, _callback));

        Ok(())
    }

    pub fn post(
        &mut self,
        path: &str,
        _callback: fn(Request, Response) -> Result<()>,
    ) -> Result<()> {
        let mut routes = self.routes.lock().unwrap();
        routes.add(Route::new(path, Method::Post, _callback));

        Ok(())
    }

    pub fn put(
        &mut self,
        path: &str,
        _callback: fn(Request, Response) -> Result<()>,
    ) -> Result<()> {
        let mut routes = self.routes.lock().unwrap();
        routes.add(Route::new(path, Method::Put, _callback));

        Ok(())
    }

    pub fn patch(
        &mut self,
        path: &str,
        _callback: fn(Request, Response) -> Result<()>,
    ) -> Result<()> {
        let mut routes = self.routes.lock().unwrap();
        routes.add(Route::new(path, Method::Patch, _callback));

        Ok(())
    }

    pub fn delete(
        &mut self,
        path: &str,
        _callback: fn(Request, Response) -> Result<()>,
    ) -> Result<()> {
        let mut routes = self.routes.lock().unwrap();
        routes.add(Route::new(path, Method::Delete, _callback));

        Ok(())
    }

    pub fn not_found(&mut self, _callback: fn(Request, Response) -> Result<()>) -> Result<()> {
        let mut routes = self.routes.lock().unwrap();
        routes.not_found = Some(_callback);

        Ok(())
    }
}

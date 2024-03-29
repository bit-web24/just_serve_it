# novanet

Lightweigh & Multithreaded web-server in rust.

# Usage

```rust
mod server;
use server::{middleware, path::Path, request::Request, response::Response, Server};
use std::io::Result;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    let mut server = Server::new("HTTP Server");

    server.not_found(not_found)?;
    server._use_(middleware::ServeStatic::dir("public"));
    server._use_(middleware::BodyParser::json());
    server._use_(middleware::BodyParser::url_encoded());

    server.get("/", root_handler)?;
    server.get("/about", about_handler)?;
    server.post("/", root_post_handler)?;

    server.listen("127.0.0.1", 8080, || {
        println!("Listening on 127.0.0.1:8080");
        Ok(())
    })?;

    Ok(())
}

fn root_handler(_req: Request, mut res: Response) -> Result<()> {
    res.send("/> Home Page!")?;
    Ok(())
}

fn about_handler(_req: Request, mut res: Response) -> Result<()> {
    sleep(Duration::from_secs(4));
    res.send("/about> About Page!")?;
    Ok(())
}

fn not_found(_req: Request, mut res: Response) -> Result<()> {
    res.status.status_code = 404;
    let path = Path::new("public");
    res.send_file(path.join("NotFound.html").to_str())?;
    Ok(())
}

fn root_post_handler(req: Request, mut res: Response) -> Result<()> {
    if let Some(body) = req.body {
        println!("RAW: {:?}", body.raw.unwrap());

        if let Some(json_data) = body.json {
            println!("JSON: {:?}", json_data);
        }

        if let Some(url_encoded_data) = body.url_encoded {
            println!("URL-ENCODED: {:?}", url_encoded_data);
        }

        let path = Path::new("public");
        res.send_file(path.join("home.html").to_str())?;
    } else {
        res.send("No Data!")?;
    }

    Ok(())
}

```

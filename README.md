# Novanet

**Novanet** is a lightweight and multithreaded `web framework` written in Rust. It is designed to handle **HTTP** requests efficiently, providing a robust foundation for building web applications and APIs. Novanet focuses on simplicity and performance, offering essential features for routing, middleware, and static file serving.

## Features

- **HTTP Request Handling:** Novanet processes HTTP requests, making it suitable for applications that do not require HTTPS.
- **Multithreaded:** Efficiently handles multiple requests simultaneously using Rust's powerful concurrency features.
- **Routing:** Define routes for different HTTP methods (GET, POST, etc.) and handle requests based on URL patterns.
- **Middleware Support:** Use middleware to process requests and responses, enabling functionalities like logging, authentication, and body parsing.
- **Static File Serving:** Serve static files from specified directories, such as HTML, CSS, and JavaScript files.
- **Error Handling:** Customizable error handling to provide user-friendly responses for different error conditions.

## Usage

Below is an example demonstrating how to use the Novanet framework.

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

## How to Run

1. Ensure you have Rust installed. If not, you can download it from [rust-lang.org](https://www.rust-lang.org/).
2. Clone the repository.
3. Navigate to the project directory.
4. Build and run the server using Cargo:
    ```sh
    cargo run
    ```
5. The server will start listening on `127.0.0.1:8080`.

## Routes

- `GET /` - Serves the home page.
- `GET /about` - Serves the about page (with a 4-second delay).
- `POST /` - Handles POST requests to the root, displaying raw, JSON, and URL-encoded body data.

## Error Handling

- A custom 404 Not Found page is served if the requested resource is not found.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.
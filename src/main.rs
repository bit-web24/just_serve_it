mod server;
use server::Server;
use server::{request::Request, response::Response};
use std::io::Result;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    let mut server = Server::new("HTTP Server");

    server.get("/", root_handler)?;

    server.get("/about", about_handler)?;

    server.listen("127.0.0.1", 8080, || {
        println!("Listening on 127.0.0.1:8080");
        Ok(())
    })?;

    Ok(())
}

fn root_handler(req: Request, mut res: Response) -> Result<()> {
    res.send("/> Home Page!")?;
    Ok(())
}

fn about_handler(req: Request, mut res: Response) -> Result<()> {
    sleep(Duration::from_secs(4));
    res.send("/about> About Page!")?;
    Ok(())
}

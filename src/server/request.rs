use super::body::Body;
use super::header::Header;
use super::method::Method;

#[derive(Clone)]
pub struct Request {
    pub method: Option<Method>,
    pub path: Option<String>,
    pub version: Option<String>,
    pub headers: Header,
    pub body: Option<Body>,
}

impl Request {
    pub fn new() -> Self {
        Self {
            method: None,
            path: None,
            version: None,
            headers: Header::new(),
            body: None,
        }
    }

    pub fn parse(request_str: &mut std::borrow::Cow<'_, str>) -> Self {
        let mut req = Self::new();

        let mut lines = request_str.lines();

        // Extract request line
        let req_ln = lines.next().unwrap();
        let mut request_line = req_ln.split_whitespace();

        req.method = Method::from(request_line.next().unwrap());
        req.path = request_line.next().unwrap().to_string().into();
        req.version = request_line.next().unwrap().to_string().into();

        // Extract headers
        let mut headers = Vec::new();
        while let Some(line) = lines.next() {
            if line.trim().is_empty() {
                break;
            }
            headers.push(line.to_string());
        }

        req.headers = Header::from(headers);

        // Extract data (if any)
        let data = lines.collect::<Vec<&str>>().join("\n");
        req.body = if data.is_empty() {
            None
        } else {
            Some(Body::new(data))
        };

        req
    }
}

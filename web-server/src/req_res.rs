use std::{collections::HashMap, fs};

pub struct Response<'a> {
    buffer: &'a [u8; 1024],
}

pub struct Request<'a> {
    buffer: &'a [u8; 1024],
}

#[derive(Debug)]
pub struct ParsedRequest {
    method: String,
    uri: String,
    http_version: f64,
    headers: HashMap<String, String>,
    body: String,
}

impl<'a> Request<'a> {
    pub fn new(buffer: &'a [u8; 1024]) -> Request<'a> {
        Request { buffer }
    }

    pub fn parse(&self) -> Option<ParsedRequest> {
        let req_str = String::from_utf8_lossy(self.buffer);

        let method;
        let uri;
        let http_version;
        let mut headers = HashMap::new();
        let mut body = String::from("");

        let mut lines: Vec<&str> = req_str.lines().collect();

        let mut parts = lines[0].split(" ");

        method = parts.next()?.to_string();
        uri = parts.next()?.to_string();
        http_version = parts.next()?.replace("HTTP/", "").parse().unwrap();

        lines.remove(0);

        for (idx, &i) in lines.iter().enumerate() {
            if i.len() == 0 {
                body = lines[idx + 1].trim().replace("\u{0}", "");
            }

            let pair: Vec<&str> = i.split(":").map(|e| e.trim()).collect();
            if pair.len() >= 2 {
                headers.insert(pair[0].to_string(), pair[1].to_string());
            }
        }

        Some(ParsedRequest {
            body,
            headers,
            http_version,
            method,
            uri,
        })
    }
}

impl<'a> Response<'a> {
    pub fn new(buffer: &'a [u8; 1024]) -> Response<'a> {
        Response { buffer }
    }
    pub fn get_page(&self) -> String {
        let req = match Request::new(&self.buffer).parse() {
            Some(v) => v,
            None => return response(400, "Bad ass Request", "", ""),
        };

        let page_path = if req.uri == "/" {
            String::from("frontend/index.html")
        } else {
            format!(
                "frontend{}{}",
                req.uri,
                if req.uri.contains(".html") {
                    ""
                } else {
                    ".html"
                }
            )
        };

        let content = fs::read_to_string(page_path)
            .unwrap_or_else(|_| fs::read_to_string("frontend/404.html").unwrap());

        response(
            200,
            "Ok",
            &format!("Content-Length: {}", content.len()),
            &content,
        )
    }
}

pub fn response(status: i32, desc: &str, headers: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {} {}\r\n{}\r\n\r\n{}",
        status, desc, headers, body
    )
}

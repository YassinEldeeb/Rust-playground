use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};
use web_server::Response;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let res = Response::new(&buffer).get_page();

    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}

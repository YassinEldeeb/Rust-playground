use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server::{Response, ThreadPool};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    let pool = ThreadPool::new(16);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let res = Response::new(&buffer).get_page();

    thread::sleep(Duration::from_millis(10));
    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}

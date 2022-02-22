use num_cpus;
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server::{Response, ThreadPool};

fn main() {
    let num_of_cpus = (num_cpus::get() as f64 * 0.8) as usize;

    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).unwrap();
    let pool = ThreadPool::new(num_of_cpus);

    println!(
        "Listening on http://{} , running on {} threads 🚀",
        addr, num_of_cpus
    );

    for stream in listener.incoming() {
        let stream = stream.expect("Couldn't establish a socket connecton!");

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream
        .read(&mut buffer)
        .expect("Couldn't read the `TcpStream` buffer!");

    let res = Response::new(&buffer).get_page();

    thread::sleep(Duration::from_millis(10));
    stream
        .write(res.as_bytes())
        .expect("Couldn't write bytes to the stream!");

    stream
        .flush()
        .expect("Couldn't write all bytes to the stream!");
}

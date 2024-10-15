use rust_cargo_practice::ThreadPool;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::{fs, thread, time::Duration};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })
    }
    println!("Shutting down server")
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if stream.read(&mut buffer).is_err() {
        eprintln!("Failed to read from stream");
        return;
    }
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 Not Found", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    if stream.write(response.as_bytes()).is_err() {
        eprintln!("Failed to write to stream")
    }
    stream.flush().unwrap();
}

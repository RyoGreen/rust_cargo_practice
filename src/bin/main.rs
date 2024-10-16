use rust_cargo_practice::ThreadPool;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::{fs, thread, time::Duration};

fn main() {
    let port = 7878;
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    let pool = ThreadPool::new(5);

    println!("HTTP server start post:{}", port);
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })
    }
    println!("Shutting down server")
}

const RESPONSE_STATUAS_OK: &str = "HTTP/1.1 200 OK";
const RESPONSE_STATUAS_NOT_FOUND: &str = "HTTP/1.1 404 Not Found";

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if stream.read(&mut buffer).is_err() {
        eprintln!("Failed to read from stream");
        return;
    }
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        (RESPONSE_STATUAS_OK, "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        (RESPONSE_STATUAS_OK, "index.html")
    } else {
        (RESPONSE_STATUAS_NOT_FOUND, "404.html")
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

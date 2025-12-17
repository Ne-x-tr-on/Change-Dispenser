use std::net::TcpListener;
use std::io::Write;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    println!("Server running on http://0.0.0.0:8000");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";
        stream.write_all(response.as_bytes()).unwrap();
    }
}

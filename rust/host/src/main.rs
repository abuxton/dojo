// https://doc.rust-lang.org/book/ch21-01-single-threaded.html

use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to 127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

// --snip--

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&stream);
//     let _http_request: Vec<_> = buf_reader
//         .lines()
//         .map(|result| result.unwrap())
//         .take_while(|line| !line.is_empty())
//         .collect();

//     let status_line = "HTTP/1.1 200 OK";

//     let length = contents.len();

//     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

//     stream.write_all(response.as_bytes()).unwrap();
// }
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    // embed the HTML at compile time (file is host/hello.html or ../hello.html relative to Cargo.toml)
    // let contents = include_str!("hello.html");
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {length}\r\nConnection: close\r\n\r\n{contents}"
    );

    stream
        .write_all(response.as_bytes())
        .expect("Failed to write response");
}

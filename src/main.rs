use std::io::{BufRead, Write};

fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:7878")
        .unwrap();
    for stream in listener
        .incoming() {
        let stream = stream.unwrap();

        handle_connections(stream);
    }
}

fn handle_connections(mut stream: std::net::TcpStream) {
    let buf_reader = std::io::BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    let contents = std::fs::read_to_string("resource/hello.html").unwrap();
    let length = contents.len();
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
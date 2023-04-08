use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

/*fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
    }
}*/

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

/*fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // 请求
    // Method Request-URI HTTP-Version CRLF
    // headers CRLF
    // message-body

    // 响应
    // HTTP-Version Status-Code Reason-Phrase CRLF
    // headers CRLF
    // message-body

    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    println!("Connection established");

    let contents = fs::read_to_string("hello.html").unwrap();
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents,);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    // println!("Request: {}", String::from_utf8_lossy(&buffer));
}*/

/*fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let status_line;
    let contents;
    let response;

    if buffer.starts_with(get) {
        status_line = "HTTP/1.1 200 OK\r\n\r\n";
        contents = fs::read_to_string("hello.html").unwrap();
        response = format!("{}{}", status_line, contents);
    } else {
        status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        contents = fs::read_to_string("404.html").unwrap();
        response = format!("{}{}", status_line, contents);
    }

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}*/

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

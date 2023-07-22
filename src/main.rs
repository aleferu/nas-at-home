use crate::html::{build_body_from_folder, build_body_from_404};
use std::{io::{Read, Write},
          net::{TcpListener, TcpStream},
          thread,
          fs::metadata};


mod html;


fn handle_client(mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap();

    loop {
        let mut buffer = [0u8; 1024];
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("{peer_addr} left.");
                    break;
                }
                let full_request: String = String::from_utf8_lossy(&buffer).to_string();
                let mut request_lines = full_request.lines();
                let get_line = request_lines.next().unwrap();
                let mut get_line_splitted = get_line.split(' ');
                if let Some(word) = get_line_splitted.next() {
                    if word != "GET" { break; }
                } else {
                    break;
                }
                let path_asked: &str;
                if let Some(word) = get_line_splitted.next() {
                    path_asked = word;
                } else {
                    break;
                }

                println!("{peer_addr}: looking to get \'{path_asked}\' served");

                // let response_body = match metadata(format!(".{path_asked}")).map_err(|err| {
                //     eprintln!("Error trying to get metadata from .{path_asked} with error: {err}");
                // }).unwrap().is_dir() {
                //     true => build_body_from_folder(path_asked),
                //     false => todo!(),
                // };
                let response: String;
                if let Ok(path_metadata) = metadata(format!(".{path_asked}")) {
                    let response_body = match path_metadata.is_dir() {
                        true => build_body_from_folder(path_asked),
                        false => todo!(),
                    };
                    response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                        response_body.len(),
                        response_body
                    );
                    println!("{peer_addr}: 200 OK");
                } else {
                    let response_body = build_body_from_404(path_asked);
                    response = format!(
                        "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n{}",
                        response_body.len(),
                        response_body
                    );
                    println!("{peer_addr}: 404 Not Found");
                }

                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();

                if buffer.starts_with(b"\r\n") {
                    break;
                }
            }
            Err (err) => {
                eprintln!("{peer_addr}: {err}");
                break;
            }
        }
    }
}


fn main() {
    let ip = "127.0.0.1";
    let port = "8080";
    let listener = TcpListener::bind(&format!("{ip}:{port}")).unwrap();
    println!("Server listening with address {ip}:{port}...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

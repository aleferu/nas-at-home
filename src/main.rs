use crate::html::{build_body_from_folder, build_body_from_404, clean_weird_chars};

use std::{io::{Read, Write},
          net::{TcpListener, TcpStream},
          thread,
          fs::{metadata, File}, eprintln,
          time::Duration};

use chrono;


mod html;


fn formatted_now_date() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}


fn handle_client(mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap().to_string();
    let peer_addr = peer_addr.split(":").next().unwrap();

    const BUFFER_SIZE: usize = 1024 * 1024;
    const SEND_DELAY: u64 = 5;
    let mut buffer = [0u8; BUFFER_SIZE];
    stream.read(&mut buffer).unwrap();
    let full_request: String = String::from_utf8_lossy(&buffer).to_string();
    let mut request_lines = full_request.lines();
    let get_line = request_lines.next().unwrap();
    let mut get_line_splitted = get_line.split(' ');
    if let Some(word) = get_line_splitted.next() {
        if word != "GET" {
            eprintln!("{date} - {peer_addr}: Invalid request sent.", date = formatted_now_date());
            return
        }
    }
    let mut path_asked = String::new();
    let order: &str;
    let asc: bool;
    if let Some(word) = get_line_splitted.next() {
        let mut word_splitted = word.split("?");
        let mut word_splitted_count = word.split("?").count();
        if word_splitted_count == 1 {
            path_asked.push_str(word);
            order = "name";
            asc = false;
        } else {
            let mut last_part = word_splitted.next().unwrap();
            word_splitted_count -= 1;
            while word_splitted_count != 0 {
                path_asked.push_str(last_part);
                last_part = word_splitted.next().unwrap();
                word_splitted_count -= 1;
            }
            let mut last_part_splitted = last_part.split("&");
            let last_part_splitted_count = last_part.split("&").count();
            if last_part_splitted_count != 2 {
                order = "name";
                asc = false;
            } else {
                order = last_part_splitted.next().unwrap().split("=").last().unwrap();
                asc = match last_part_splitted.next().unwrap().split("=").last().unwrap() {
                    "true" => true,
                    "false" | _ => false,
                };
            }
        }
    } else {
        eprintln!("{date} - {peer_addr}: something went wrong. Weird request.", date = formatted_now_date());
        return
    }
    let path_asked = clean_weird_chars(path_asked);
    let response: String;
    let mut is_file: bool = false;
    if let Ok(path_metadata) = metadata(format!(".{path_asked}")) {
        if path_metadata.is_dir() {
            let response_body = build_body_from_folder(&path_asked, order, asc);
            response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\n\r\n{}",
                response_body.len(),
                response_body
            );
        } else {
            let future_file_name = path_asked.split('/').last().unwrap();
            response = format!(
                "HTTP/1.1 200 OK\r\nContent-Disposition: attachment; filename=\"{}\"\r\nContent-type: application/octet-stream\r\nContent-Length: {}\r\n\r\n",
                future_file_name,
                path_metadata.len(),
            );
            is_file = true;
        }
        println!("{date} - {peer_addr}: 200 OK {path_asked}", date = formatted_now_date());
    } else {
        let response_body = build_body_from_404(&path_asked);
        response = format!(
            "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\n\r\n{}",
            response_body.len(),
            response_body
        );
        println!("{date} - {peer_addr}: 404 Not Found {path_asked}", date = formatted_now_date());
    }

    stream.write(response.as_bytes()).unwrap();
    if is_file { 
        let mut file = File::open(format!(".{path_asked}")).unwrap();
        loop {
            match file.read(&mut buffer) {
                Ok(0) => break,
                Ok(bytes_read) => {
                    stream.write_all(&buffer[..bytes_read]).map_err(|err| {
                        eprintln!("{peer_addr}: {err}");
                    }).unwrap();
                    thread::sleep(Duration::from_millis(SEND_DELAY));
                },
                Err(err) => {
                    eprintln!("{date} - {peer_addr}: Error reading file: {err}", date = formatted_now_date());
                    break
                },
            }
        }
    }
    match stream.flush() {
        Ok(_) => {},
        Err(err) => { eprintln!("{date} - {peer_addr}: {err}", date = formatted_now_date()); },
    }
}


fn main() {
    let ip = "127.0.0.1";
    let port = "8080";
    let listener = TcpListener::bind(&format!("{ip}:{port}")).unwrap();
    println!("Server listening with address {ip}:{port}...\n");

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

use crate::{html::{build_body_from_folder, build_body_from_404, clean_weird_chars},
            args_reader::*};

use std::{io::{Read, Write},
          net::{TcpListener, TcpStream},
          thread,
          fs::{metadata, File}, eprintln,
          time::Duration};

use chrono;


mod html;
mod args_reader;


fn formatted_now_date() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}


fn handle_client(mut stream: TcpStream, starting_path: String) {
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
    let mut path_asked = starting_path.clone();
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
    if let Ok(path_metadata) = metadata(&path_asked) {
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
        let mut file = File::open(format!("{path_asked}")).unwrap();
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
    if look_for_flag("help") {
        println!("Usage: nas-at-home [FLAGS] [OPTIONS]");
        println!();
        println!("FLAGS:");
        println!("    --help     Prints this, nothing else happens.");
        println!("OPTIONS");
        println!("    --ip      Sets the ip for the TCP Listener, 127.0.0.1 is the default value.");
        println!("              Example: --ip 127.0.0.1");
        println!("    --port    Sets the port for the TCP Listener, 8080 is the default value.");
        println!("              Example: --port 8080");
        println!("    --path    Sets the root folder, . is the default value.");
        println!("              Example: --path ./src/");
        println!();
    } else {
        let ip = look_for_option("ip").unwrap_or("127.0.0.1".to_string());
        let port = look_for_option("port").unwrap_or("8080".to_string());
        let mut starting_path = look_for_option("path").unwrap_or(".".to_string());
        if starting_path != "/" && starting_path.ends_with("/") {
            starting_path = starting_path[..starting_path.len() - 1].to_string();
        }
        match TcpListener::bind(&format!("{ip}:{port}")) {
            Ok(listener) => {
                println!("Server listening with address {ip}:{port}...\n");
                println!("Server started at path {starting_path}");

                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => { 
                            let starting_path_clone = starting_path.clone();
                            thread::spawn(|| handle_client(stream, starting_path_clone)); 
                        }
                        Err(err) => { eprintln!("Error handling an incoming stream: {err}"); }
                    }
                }
            }
            Err(err) => { eprintln!("Error trying to create the server: {err}") }
        }
    }
}

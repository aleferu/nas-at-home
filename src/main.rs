use crate::{html::{build_body_from_folder, build_body_from_404},
            args_reader::*,
            http_parser::Request};

use std::{io::{Read, Write},
          net::{TcpListener, TcpStream},
          thread,
          fs::{metadata, File},
          time::Duration,
          collections::HashMap};

use chrono;


mod html;
mod args_reader;
mod http_parser;


fn formatted_now_date() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}


fn handle_get_request(stream: &mut TcpStream, path_asked: &str, starting_path: &str, peer_ip: &str, buffer: &mut [u8], options: &Option<HashMap<String, String>>) {
    let response: String;
    let mut is_file: bool = false;
    let mut full_path = starting_path.to_string();
    full_path.push_str(path_asked);
    if let Ok(file_metadata) = metadata(&full_path) {
        if file_metadata.is_dir() {
            let (order, asc) = match options {
                Some(options) => {
                    let order = options.get("order").unwrap_or(&"name".to_string()).clone();
                    let asc = options.get("asc").unwrap_or(&"true".to_string()).clone();
                    (order, asc == "true")
                },
                None => { ("name".to_string(), true) },
            };
            let response_body = build_body_from_folder(starting_path, path_asked, &order, asc);
            response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\nCache-Control: no-cache, no-store, must-revalidate\r\nPragma: no-cache\r\nExpires: 0\r\n\r\n{}",
                response_body.len(),
                response_body
            );
        } else {
            let future_file_name = path_asked.split('/').last().unwrap();
            response = format!(
                "HTTP/1.1 200 OK\r\nContent-Disposition: attachment; filename=\"{}\"\r\nCache-Control: no-cache, no-store, must-revalidate\r\nPragma: no-cache\r\nExpires: 0\r\nContent-type: application/octet-stream\r\nContent-Length: {}\r\n\r\n",
                future_file_name,
                file_metadata.len(),
            );
            is_file = true;
        }
        println!("{date} - {peer_ip}: 200 OK {path_asked}", date = formatted_now_date());
    } else {
        let response_body = build_body_from_404(&path_asked);
        response = format!(
            "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\nContent-Type: text/html; charset=utf-8\r\n\r\n{}",
            response_body.len(),
            response_body
        );
        println!("{date} - {peer_ip}: 404 Not Found {path_asked}", date = formatted_now_date());
    }

    stream.write_all(response.as_bytes()).map_err(|err| {
        eprintln!("{peer_ip}: {err}");
    }).unwrap();
    if is_file {
        let send_delay = 3;
        let mut file = File::open(full_path).unwrap();
        loop {
            match file.read(buffer) {
                Ok(0) => break,
                Ok(bytes_read) => {
                    stream.write_all(&buffer[..bytes_read]).map_err(|err| {
                        eprintln!("{peer_ip}: {err}");
                    }).unwrap();
                    thread::sleep(Duration::from_millis(send_delay));
                },
                Err(err) => {
                    eprintln!("{date} - {peer_ip}: Error reading file: {err}", date = formatted_now_date());
                    break
                },
            }
        }
    }
    match stream.flush() {
        Ok(_) => {},
        Err(err) => { eprintln!("{date} - {peer_ip}: {err}", date = formatted_now_date()); },
    }
}


fn handle_client(mut stream: TcpStream, starting_path: String) {
    let peer_addr = stream.peer_addr().unwrap().to_string();
    let peer_ip = peer_addr.split(":").next().unwrap();

    const BUFFER_SIZE: usize = 100 * 1024;
    let mut buffer = [0u8; BUFFER_SIZE];
    stream.read(&mut buffer).unwrap();
    match Request::from(&buffer) {
        Request::Get { path: path_asked, options } => { 
            println!("{date} - {peer_ip}: GET {path_asked}", date = formatted_now_date());
            handle_get_request(&mut stream, &path_asked, &starting_path, peer_ip, &mut buffer, &options)
        },
        Request::Unsupported => { eprintln!("{date} - {peer_ip}: Invalid request received.", date = formatted_now_date()) },
    };
}


fn main() {
    if look_for_flag("help") {
        println!("Usage: nas-at-home [FLAGS] [OPTIONS]");
        println!();
        println!("FLAGS:");
        println!("  --help   Prints this, nothing else happens.");
        println!("OPTIONS");
        println!("  --ip      Sets the ip for the TCP Listener, 127.0.0.1 is the default value.");
        println!("            Example: --ip 127.0.0.1");
        println!("  --port    Sets the port for the TCP Listener, 8080 is the default value.");
        println!("            Example: --port 8080");
        println!("  --path    Sets the root folder, . is the default value.");
        println!("            Example: --path /home/");
        println!();
    } else {
        let ip = look_for_option("ip").unwrap_or("127.0.0.1".to_string());
        let port = look_for_option("port").unwrap_or("8080".to_string());
        let mut starting_path = look_for_option("path").unwrap_or(".".to_string());
        if starting_path != "/" && starting_path.ends_with("/") {
            starting_path = starting_path[..starting_path.len() - 1].to_string();
        }
        if cfg!(windows) && starting_path == "/" {
            eprintln!("Not a linux system. Please try a different path");
            return ()
        }
        if let Ok(path_metadata) = metadata(&starting_path) {
            if !path_metadata.is_dir() {
                eprintln!("Could not start the server using the following path: {starting_path}");
                eprintln!("Is it a folder?");
                return ()
            }
        } else {
            eprintln!("Could not start the server using the following path: {starting_path}");
            eprintln!("Is it a valid path?");
            return ()
        }
        match TcpListener::bind(&format!("{ip}:{port}")) {
            Ok(listener) => {
                println!("Server listening with address {ip}:{port}...\n");
                println!("Server started at path {starting_path}");

                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => { 
                            let starting_path_clone = starting_path.clone();
                            thread::spawn(move || handle_client(stream, starting_path_clone)); 
                        }
                        Err(err) => { eprintln!("Error handling an incoming stream: {err}"); }
                    }
                }
            }
            Err(err) => { eprintln!("Error trying to create the server: {err}") }
        }
    }
}

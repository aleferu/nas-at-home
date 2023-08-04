use std::{time::SystemTime,
          ffi::OsString,
          fs::read_dir};
use chrono;


pub fn build_body_from_folder(starting_path: &str, folder_path: &str, order: &str, asc: bool, upload: bool) -> String {
    let mut response_body = html_start_head();
    if upload {
        response_body.push_str(&upload_part(folder_path));
    }
    response_body.push_str(&folder_name_part(starting_path, folder_path));
    response_body.push_str(&order_by_part(order, asc));
    response_body.push_str(&list_elements(folder_path, order, asc));
    response_body.push_str(&html_end(false));
    response_body
}


pub fn build_body_from_404(failed_path: &str) -> String {
    let mut response_body = html_start_head();
    response_body.push_str(&format!(r#"
<p>Error trying to find {failed_path}.</p>
<p>If this error should not have happpened, please contact me so I can fix it.</p>
<div><a href="https://twitter.com/aleferu_"><strong>Twitter</strong></a></div>
<div><a href="https://github.com/aleferu"><strong>GitHub</strong></a></div>
<div><a href="https://github.com/aleferu/nas-at-home"><strong>GitHub repo</strong></a></div>
<p>Maybe go back to <a href="/"><strong>root</strong></a> in the meantime?</p>
"#));
    response_body.push_str(&html_end(true));
    response_body
}


pub fn clean_weird_chars(input: String) -> String{
    let mut decoded_string = String::new();
    let mut bytes = Vec::new();

    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            let chars_clone = chars.clone();
            let next_char: String = chars_clone.take(1).collect();
            if next_char == "%" {
                decoded_string.push(c);
            } else {
                let hex_str: String = chars.by_ref().take(2).collect();
                if let Ok(byte) = u8::from_str_radix(&hex_str, 16) {
                    bytes.push(byte);
                } else {
                    decoded_string.push(c);
                    decoded_string.push_str(&hex_str);
                }
            }
        } else {
            if !bytes.is_empty() {
                decoded_string.push_str(&String::from_utf8_lossy(&bytes));
                bytes.clear();
            }
            decoded_string.push(c);
        }
    }

    if !bytes.is_empty() {
        decoded_string.push_str(&String::from_utf8_lossy(&bytes));
    }

    decoded_string
}


struct Element {
    name: OsString,
    last_mod: SystemTime,
    size: u64
}


fn html_start_head() -> String {
    String::from(r#"<!DOCTYPE html>
<html>
  <head>
    <title>nas-at-home</title>
    <meta charset="utf-8"/>
    <meta name="author" content="aleferu"/>
    <meta http-equiv="cache-control" content="no-cache"/>
    <meta name="viewport" content="width=device-width,initial-scale=1.0, minimum-scale=1.0, maximum-scale=1.0, user-scalable=no"/>
    <style type="text/css">a { text-decoration:none; color:blue; }</style>
    <style type="text/css">td { padding: 0 15px; }</style>
  </head>
  <body>"#)
}


// TODO
fn upload_part(_folder_path: &str) -> String {
    String::from(r#"
<form style="margin-top:1em; margin-bottom:1em;" action="
"#)
}


fn folder_name_part(starting_path: &str, folder_path: &str) -> String {
    let mut current_path = String::from("/");
    let mut result = format!(r#"
<div>
  <a href="{current_path}"><strong>ROOT</strong></a>
    "#);
    let mut folder_path_splitted = folder_path.split('/');
    if starting_path.starts_with('/') {
        let mut starting_path_splitted = starting_path.split('/');
        while let Some(_) = starting_path_splitted.next() {
            folder_path_splitted.next();
        }
    } else {
        folder_path_splitted.next();
    }
    for folder in folder_path_splitted {
        if folder == "" { continue }
        current_path.push_str(&format!("{folder}/"));
        result.push_str(" / ");
        result.push_str(&format!("<a href=\"{current_path}\"><strong>{folder}</strong></a>\r\n"));
    }
    result.push_str("</div>\r\n<hr />\r\n");
    result
}


fn order_by_part(order: &str, asc: bool) -> String {
    let asc_name = if order == "name" { (!asc).to_string() } else { "false".to_string() };
    let asc_modified = if order == "modified" { (!asc).to_string() } else { "false".to_string() };
    let asc_size = if order == "size" { (!asc).to_string() } else { "false".to_string() };
    format!(r#"
<table>
  <tr>
    <th><a href="?order=name&asc={asc_name}">Name</a></th>
    <th><a href="?order=modified&asc={asc_modified}">Last modified</a></th>
    <th><a href="?order=size&asc={asc_size}">Size</a></th>
    <th><a href="?order=size&asc={asc_size}">Real Byte Count</a></th>
  </tr>"#)
}


fn order_elements_by(v: &mut Vec<Element>, order: &str, asc: bool) {
    v.sort_by(|a, b| {
        match order {
            "modified" => {
                let mut comparison = a.last_mod.cmp(&b.last_mod);
                if !asc {
                    comparison = comparison.reverse();
                }
                comparison
            },
            "size" => {
                let mut comparison = a.size.cmp(&b.size);
                if !asc {
                    comparison = comparison.reverse();
                }
                comparison
            },
            "name" | _ => {
                let mut comparison = a.name.cmp(&b.name);
                if !asc {
                    comparison = comparison.reverse();
                }
                comparison
            },
        }
    })
}


fn readable_bytes(bytes: u64) -> String {
    let mut bytes = bytes as f64;
    let prefixes = " KMGTPEZY";
    let mut counter = 0usize;
    while bytes >= 1024f64 {
        bytes /= 1024f64;
        counter += 1;
    }
    if bytes == bytes.round() {
        format!("{} {}B", bytes, prefixes.chars().nth(counter).expect("Biggest file ever."))
    } else {
        format!("{:.3} {}B", bytes, prefixes.chars().nth(counter).expect("Biggest file ever."))
    }
}


fn readable_last_mod(last_mod: SystemTime) -> String {
    let local_last_mod: chrono::DateTime<chrono::Local> = last_mod.into();
    local_last_mod.format("%Y-%m-%d %H:%M:%S").to_string()
}


fn list_elements(folder_path: &str, order: &str, asc: bool) -> String {
    match read_dir(folder_path) {
        Ok(mut entries) => {
            let mut directories: Vec<Element> = Vec::new();
            let mut files: Vec<Element> = Vec::new();
            while let Some(entry) = entries.next() {
                if entry.is_err() {
                    continue
                }
                let entry = entry.unwrap();
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(last_mod) = metadata.modified() {
                        if metadata.is_dir() {
                            directories.push(Element{
                                name: entry.file_name(), 
                                last_mod: last_mod, 
                                size: metadata.len()
                            });
                        } else {
                            files.push(Element{
                                name: entry.file_name(),
                                last_mod: last_mod,
                                size: metadata.len()
                            });
                        }
                    }
                }
            }
            order_elements_by(&mut directories, order, asc);
            order_elements_by(&mut files, order, asc);
            let mut result = String::new();
            for directory in directories {
                let name = &directory.name.to_str().unwrap();
                let last_mod = readable_last_mod(directory.last_mod);
                let bytes = readable_bytes(directory.size);
                result.push_str(&format!(r#"
<tr>
  <td><a style="font-weight: bold;" href="{}/">{}</a></td>
  <td style="color:#888;">{}</td>
  <td><bold>{}</bold></td>
  <td><bold>{}</bold></td>
</tr>
"#, name, name, last_mod, bytes, directory.size));
            }
            for file in files {
                let name = &file.name.to_str().unwrap();
                let last_mod = readable_last_mod(file.last_mod);
                let bytes = readable_bytes(file.size);
                result.push_str(&format!(r#"
<tr>
  <td><a href="{}">{}</a></td>
  <td style="color:#888;">{}</td>
  <td><bold>{}</bold></td>
  <td><bold>{}</bold></td>
</tr>
"#, name, name, last_mod, bytes, file.size));
            }
            result
        },
        Err(err) => {
            eprintln!("Error trying to get the entries of the following path: {folder_path}");
            eprintln!("Error given by the system: {err}");
            String::from("")
        }
    }
}


fn html_end(is_404: bool) -> String {
    let mut result = match is_404 {
        true => String::new(),
        false => String::from("\r\n</table>\r\n"),
    };
    result.push_str("</body>\r\n</html>");
    result
}

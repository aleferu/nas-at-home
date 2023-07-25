use std::{time::SystemTime,
          ffi::OsString,
          fs::read_dir};
use chrono;


struct Element {
    name: OsString,
    last_mod: SystemTime,
    size: u64
}


pub fn build_body_from_folder(folder_path: &str, order: &str, asc: bool) -> String {
    let mut response_body = html_start_head();
    response_body.push_str(&folder_name_part(folder_path));
    response_body.push_str(&order_by_part(folder_path, order, asc));
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


fn folder_name_part(folder_path: &str) -> String {
    let mut current_path = String::from("/");
    let mut result = format!(r#"
<div>
  <a href="{current_path}"><strong>ROOT</strong></a>
    "#);
    let folder_path_splitted = folder_path.split('/');
    for folder in folder_path_splitted {
        if folder == "" { continue }
        current_path.push_str(&format!("{folder}/"));
        result.push_str(" / ");
        result.push_str(&format!("<a href=\"{current_path}\"><strong>{folder}</strong></a>\r\n"));
    }
    result.push_str("</div>\r\n<hr />\r\n");
    result
}


fn order_by_part(folder_path: &str, order: &str, asc: bool) -> String {
    let asc_name = if order == "name" { (!asc).to_string() } else { "false".to_string() };
    let asc_modified = if order == "modified" { (!asc).to_string() } else { "false".to_string() };
    let asc_size = if order == "size" { (!asc).to_string() } else { "false".to_string() };
    format!(r#"
<table>
  <tr>
    <th><a href="{folder_path}?order=name&asc={asc_name}">Name</a></th>
    <th><a href="{folder_path}?order=modified&asc={asc_modified}">Last modified</a></th>
    <th><a href="{folder_path}?order=size&asc={asc_size}">Size</a></th>
    <th><a href="{folder_path}?order=size&asc={asc_size}">Real Bytes</a></th>
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
    let real_path = format!(".{folder_path}");
    let mut directories: Vec<Element> = Vec::new();
    let mut files: Vec<Element> = Vec::new();
    for entry in read_dir(real_path).unwrap() {
        if let Ok(entry) = entry {
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
}


fn html_end(is_404: bool) -> String {
    let mut result = match is_404 {
        true => String::new(),
        false => String::from("\r\n</table>\r\n"),
    };
    result.push_str("</body>\r\n</html>");
    result
}

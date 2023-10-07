use std::collections::HashMap;


pub enum Request {
    Get { 
        path: String,
        options: Option<HashMap<String, String>>,
    },
    Unsupported,
}


impl Request {

    pub fn from(request: &[u8]) -> Request {
        let sep: &[u8] = b"\r\n";
        if request.len() <= "GET /".len() {
            return Request::Unsupported;
        }
        let mut request_content: Vec<Vec<u8>> = Vec::new();
        let mut aux = Vec::new();
        let mut new_line = false;
        for window in request.windows(sep.len()) {
            if window != sep {
                if new_line {
                    new_line = false;
                    continue;
                }
                aux.push(window[0]);
            } else {
                new_line = true;
                request_content.push(aux.clone());
                aux.clear();
            }
        }
        let mut request_content = request_content.iter_mut();
        let request_line = String::from_utf8(request_content.next().unwrap().to_vec()).unwrap();
        let mut request_line_splitted = request_line.split(' ');
        let request_method: &str = request_line_splitted.next().unwrap();
        if let Some(path) = request_line_splitted.next() {
            if request_method == "GET" {
                let cleaned_path = clean_weird_chars(path.to_string());
                if let Some((path_asked, query_params)) = parse_url(&cleaned_path) {
                    return Request::Get { path: path_asked, options: query_params }
                }
            }
            else if request_method == "POST" {
                todo!()
            }
        }
        return Request::Unsupported;
    }
}


fn clean_weird_chars(input: String) -> String{
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


fn parse_url(full_url: &str) -> Option<(String, Option<HashMap<String, String>>)> {
    let mut parts = full_url.splitn(2, '?');
    let parts_count = parts.clone().count();
    if parts_count < 2 {
        return Some((full_url.to_string(), None))
    }
    let path = parts.next()?.to_string();
    let query = parts.next()?;
    let mut query_params = HashMap::new();
    for param in query.split('&') {
        let mut param_parts = param.splitn(2, '=');
        let key = param_parts.next()?.to_string();
        let value = param_parts.next().unwrap_or("").to_string();
        query_params.insert(key, value);
    }
    Some((path, Some(query_params)))
}

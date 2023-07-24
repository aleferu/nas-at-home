


pub fn build_body_from_folder(folder_path: &str, order: &str, asc: bool) -> String {
    let mut response_body = html_start_head();
    response_body.push_str(&folder_name_part(folder_path));
    response_body.push_str(&order_by_part(folder_path, order, asc));
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
    <style> a { text-decoration:none; }</style>
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
  </tr>
    "#)
}


fn html_end(is_404: bool) -> String {
    let mut result = match is_404 {
        true => String::new(),
        false => String::from("</table>\r\n"),
    };
    result.push_str("</body>\r\n</html>");
    result
}

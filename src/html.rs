


pub fn html_start_head() -> String {
    return r#"<!DOCTYPE html>
<html>
  <head>
    <title>nas-at-home</title>
    <meta charset="utf-8"/>
    <meta name="author" content="aleferu"/>
    <meta http-equiv="cache-control" content="no-cache"/>
  </head>
  <body>
    "#.to_string()
}


pub fn html_end() -> String {
    return r#"</body></html>
    "#.to_string();
}

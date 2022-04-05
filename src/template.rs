use handlebars::{Handlebars, RenderError};
use serde::Serialize;

#[derive(Serialize)]
struct Context {
    title: String,
    body: String,
}

pub fn simple(title: String, body: String) -> Result<String, RenderError> {
    const TEMPLATE: &str = r#"<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
  <meta http-equiv="Content-Type" content="text/html; charset=utf-8" />
  <meta http-equiv="Content-Style-Type" content="text/css" />
  <meta name="generator" content="unidoc" />
  <title>{{title}}</title>
  <style type="text/css">code{white-space: pre;}</style>
</head>
<body>
{{{body}}}
</body>
</html>"#;
    let reg = Handlebars::new();
    let context = Context { title, body };
    reg.render_template(TEMPLATE, &context)
}
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
  <link rel="stylesheet" href="https://cympfh.cc/resources/css/youtube.css" />
  <link href="https://unpkg.com/prismjs@1.x.0/themes/prism.css" rel="stylesheet" />
  <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
  <script id="MathJax-script" async src="https://unpkg.com/mathjax@3/es5/tex-mml-chtml.js"></script>
</head>
<body>
{{{body}}}
  <script src="https://cympfh.cc/resources/js/youtube.js"></script>
  <script src="https://unpkg.com/prismjs@v1.x/components/prism-core.min.js"></script>
  <script src="https://unpkg.com/prismjs@v1.x/plugins/autoloader/prism-autoloader.min.js"></script>
</body>
</html>"#;
    let reg = Handlebars::new();
    let context = Context { title, body };
    reg.render_template(TEMPLATE, &context)
}

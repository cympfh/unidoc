use handlebars::{Handlebars, RenderError};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Context {
    title: String,
    body: String,
    css: Vec<String>,
    headers: Vec<String>,
    befores: Vec<String>,
    afters: Vec<String>,
    variable: HashMap<String, String>,
}

impl Context {
    pub fn new(
        title: String,
        body: String,
        css: Vec<String>,
        headers: Vec<String>,
        befores: Vec<String>,
        afters: Vec<String>,
        variable: HashMap<String, String>,
    ) -> Self {
        Self {
            title,
            body,
            css,
            headers,
            befores,
            afters,
            variable,
        }
    }
}

pub fn simple(context: Context) -> Result<String, RenderError> {
    const TEMPLATE: &str = r#"<!DOCTYPE html>
<html>
<head>
  <meta http-equiv="Content-Type" content="text/html; charset=utf-8" />
  <meta http-equiv="Content-Style-Type" content="text/css" />
  <meta name="generator" content="unidoc" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=yes">
  <title>{{{title}}}</title>
  <style type="text/css">code{white-space: pre;}</style>
  <link rel="stylesheet" href="https://cympfh.cc/resources/css/youtube.css" />
  <link href="https://unpkg.com/prismjs@1.x.0/themes/prism.css" rel="stylesheet" />
  <script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
  <script id="MathJax-script" async src="https://unpkg.com/mathjax@3/es5/tex-svg-full.js"></script>
  {{#each css}}
  <link href="{{this}}" rel="stylesheet" />
  {{/each}}
{{#each headers}}{{{this}}}{{/each}}
</head>
<body>
{{#each befores}}{{{this}}}{{/each}}
{{{body}}}
{{#each afters}}{{{this}}}{{/each}}
  <script src="https://cympfh.cc/resources/js/youtube.js"></script>
  <script src="https://unpkg.com/prismjs@v1.x/components/prism-core.min.js"></script>
  <script src="https://unpkg.com/prismjs@v1.x/plugins/autoloader/prism-autoloader.min.js"></script>
</body>
</html>"#;
    let reg = Handlebars::new();
    reg.render_template(TEMPLATE, &context)
}

pub fn custom(htmltemplate: &str, context: Context) -> Result<String, RenderError> {
    let reg = Handlebars::new();
    reg.render_template(htmltemplate, &context)
}
